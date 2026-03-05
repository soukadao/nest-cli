use anyhow::Result;
use nest_core::repo::Repository;

pub fn run(repo: &Repository) -> Result<()> {
    let snapshots = repo.snapshot_list()?;
    if snapshots.is_empty() {
        println!("No snapshots in history.");
        return Ok(());
    }

    for snap in snapshots.iter().rev() {
        println!("snapshot {}", snap.id.as_str());
        println!("Author: {}", snap.author);
        println!("Group:  {}", snap.group.as_str());
        println!();
        println!("    {}", snap.message);
        println!();
    }

    Ok(())
}
