use anyhow::Result;
use nest_core::repo::Repository;

pub fn run(_repo: &Repository, _from: Option<&str>, _to: Option<&str>) -> Result<()> {
    // TODO: Implement snapshot diff
    println!("Diff not yet implemented.");
    Ok(())
}
