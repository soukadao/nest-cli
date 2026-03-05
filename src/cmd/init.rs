use anyhow::Result;
use nest_core::repo::Repository;
use std::path::Path;

pub fn run(path: &Path, user_name: &str) -> Result<()> {
    let repo = Repository::init(path, user_name)?;
    println!("Initialized empty Nest repository in {}", repo.root().display());
    println!("  Node ID: {:032x}", repo.config.node_id);
    println!("  User: {}", repo.config.user.name);
    println!("  Default branch: main");
    Ok(())
}
