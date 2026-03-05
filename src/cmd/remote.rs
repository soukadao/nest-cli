use anyhow::Result;
use nest_core::config::RemoteConfig;
use nest_core::repo::Repository;

pub fn add(repo: &mut Repository, name: &str, url: &str) -> Result<()> {
    repo.config.remotes.insert(
        name.to_string(),
        RemoteConfig {
            name: name.to_string(),
            url: url.to_string(),
            token: None,
        },
    );
    repo.config.save(&repo.nest_dir().join("config.json"))?;
    println!("Added remote '{}' -> {}", name, url);
    Ok(())
}

pub fn list(repo: &Repository) -> Result<()> {
    if repo.config.remotes.is_empty() {
        println!("No remotes configured.");
        return Ok(());
    }
    for (name, remote) in &repo.config.remotes {
        println!("  {} -> {}", name, remote.url);
    }
    Ok(())
}

pub fn remove(repo: &mut Repository, name: &str) -> Result<()> {
    if repo.config.remotes.remove(name).is_some() {
        repo.config.save(&repo.nest_dir().join("config.json"))?;
        println!("Removed remote '{}'", name);
    } else {
        println!("Remote '{}' not found", name);
    }
    Ok(())
}
