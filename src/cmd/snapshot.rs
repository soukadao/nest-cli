use anyhow::Result;
use nest_core::repo::Repository;

pub fn create(repo: &mut Repository, message: &str) -> Result<()> {
    let snapshot = repo.snapshot_create(message)?;
    println!("Created snapshot: {}", snapshot.id.as_str());
    println!("  Message: {}", snapshot.message);
    println!("  Author: {}", snapshot.author);
    println!("  Tree hash: {}...", &snapshot.tree_hash[..16]);
    Ok(())
}

pub fn list(repo: &Repository) -> Result<()> {
    let snapshots = repo.snapshot_list()?;
    if snapshots.is_empty() {
        println!("No snapshots.");
        return Ok(());
    }
    for snap in snapshots.iter().rev() {
        println!(
            "  {} - {} (by {})",
            &snap.id.as_str(),
            snap.message,
            snap.author,
        );
    }
    Ok(())
}
