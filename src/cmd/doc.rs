use anyhow::{Context, Result};
use nest_core::repo::Repository;
use nest_protocol::entity::EntityId;

pub fn create(repo: &mut Repository, title: &str, body: Option<&str>) -> Result<()> {
    let body_text = match body {
        Some(b) => b.to_string(),
        None => open_editor("")?,
    };

    let id = repo.doc_create(title, &body_text)?;
    println!("Created document: {} ({})", title, id.as_str());
    Ok(())
}

pub fn list(repo: &Repository) -> Result<()> {
    let docs = repo.doc_list()?;
    if docs.is_empty() {
        println!("No documents.");
        return Ok(());
    }
    for doc in &docs {
        println!(
            "  [{}] {} (by {})",
            doc.id.as_str(),
            doc.title.get(),
            doc.created_by,
        );
    }
    Ok(())
}

pub fn show(repo: &Repository, id: &str) -> Result<()> {
    let doc = repo.doc_get(&EntityId::new(id))?;
    println!("# {}", doc.title.get());
    println!();
    println!("{}", doc.body.to_string());
    Ok(())
}

pub fn edit(repo: &mut Repository, id: &str) -> Result<()> {
    let entity_id = EntityId::new(id);
    let doc = repo.doc_get(&entity_id)?;
    let old_body = doc.body.to_string();

    let new_body = open_editor(&old_body)?;

    if new_body == old_body {
        println!("No changes.");
        return Ok(());
    }

    repo.doc_update_body(&entity_id, &new_body)?;
    println!("Updated document: {}", doc.title.get());
    Ok(())
}

fn open_editor(initial_content: &str) -> Result<String> {
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
    let tmp_dir = std::env::temp_dir();
    let tmp_path = tmp_dir.join(format!("nest-doc-{}.md", std::process::id()));

    std::fs::write(&tmp_path, initial_content)?;

    let status = std::process::Command::new(&editor)
        .arg(&tmp_path)
        .status()
        .context(format!("Failed to launch editor: {editor}"))?;

    if !status.success() {
        anyhow::bail!("Editor exited with non-zero status");
    }

    let content = std::fs::read_to_string(&tmp_path)?;
    let _ = std::fs::remove_file(&tmp_path);
    Ok(content)
}
