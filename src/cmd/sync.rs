use anyhow::{Context, Result};
use nest_core::repo::Repository;
use nest_protocol::entity::StreamGroupId;
use nest_protocol::messages::StreamPosition;

use crate::sync_client::SyncClient;

pub async fn run(repo: &mut Repository, remote: Option<&str>) -> Result<()> {
    let remote_name = remote.unwrap_or("origin");

    let remote_config = repo
        .config
        .remotes
        .get(remote_name)
        .context(format!("Remote '{}' not found", remote_name))?
        .clone();

    let group_id_str = repo
        .config
        .current_group
        .as_ref()
        .context("No current branch")?
        .clone();
    let group_id = StreamGroupId::new(&group_id_str);

    // Collect local streams and their ops
    let stream_ids = repo.stream_store.list_streams(Some(&group_id))?;
    let mut local_streams = Vec::new();
    let mut known_positions = Vec::new();

    for sid in &stream_ids {
        let stream = repo.stream_store.read_stream(sid)?;
        known_positions.push(StreamPosition {
            stream_id: sid.clone(),
            op_index: stream.ops.len(),
        });
        if !stream.ops.is_empty() {
            local_streams.push((sid.clone(), stream.ops.clone()));
        }
    }

    let local_op_count: usize = local_streams.iter().map(|(_, ops)| ops.len()).sum();
    println!(
        "Syncing with {} ({} local ops)...",
        remote_config.url, local_op_count
    );

    // Build sync URL
    // Determine repo name from the root directory name
    let repo_name = repo
        .root()
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "default".to_string());

    let sync_url = format!(
        "{}/api/repos/{}/sync",
        remote_config.url.trim_end_matches('/'),
        repo_name
    );

    let client = SyncClient::new(&sync_url, repo.config.node_id)
        .with_token(remote_config.token);

    let remote_ops = client
        .sync(&group_id, local_streams, known_positions)
        .await?;

    // Apply remote ops
    let mut remote_op_count = 0;
    for (stream_id, ops) in &remote_ops {
        remote_op_count += ops.len();
        repo.apply_remote_ops(ops)?;
        println!(
            "  Received {} ops from stream {}",
            ops.len(),
            stream_id.as_str()
        );
    }

    if remote_op_count == 0 && local_op_count == 0 {
        println!("Already up to date.");
    } else {
        println!(
            "Sync complete: sent {} ops, received {} ops.",
            local_op_count, remote_op_count
        );
    }

    Ok(())
}
