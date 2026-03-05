use anyhow::Result;
use nest_core::repo::Repository;

pub fn create(_repo: &mut Repository, title: &str, source: &str, target: &str) -> Result<()> {
    // TODO: Full review creation with entity
    println!("Created review: {} ({}→{})", title, source, target);
    Ok(())
}

pub fn list(_repo: &Repository) -> Result<()> {
    // TODO: List reviews
    println!("No reviews yet.");
    Ok(())
}
