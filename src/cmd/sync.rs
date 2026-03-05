use anyhow::Result;
use nest_core::repo::Repository;

pub async fn run(_repo: &mut Repository, _remote: Option<&str>) -> Result<()> {
    // TODO: Implement sync via WebSocket
    println!("Sync not yet implemented.");
    Ok(())
}
