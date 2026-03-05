use anyhow::Result;
use nest_core::repo::Repository;

pub fn list(repo: &Repository) -> Result<()> {
    let groups = repo.group_list()?;
    if groups.is_empty() {
        println!("No branches.");
        return Ok(());
    }
    for group in groups {
        let current = repo
            .config
            .current_group
            .as_ref()
            .is_some_and(|g| g == group.id.as_str());
        let marker = if current { " *" } else { "" };
        println!("  {}{}", group.name, marker);
    }
    Ok(())
}

pub fn create(repo: &mut Repository, name: &str) -> Result<()> {
    repo.group_create(name)?;
    println!("Created branch '{}'", name);
    Ok(())
}

pub fn switch(repo: &mut Repository, name: &str) -> Result<()> {
    repo.group_switch(name)?;
    println!("Switched to branch '{}'", name);
    Ok(())
}
