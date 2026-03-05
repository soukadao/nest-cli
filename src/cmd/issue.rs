use anyhow::Result;
use nest_core::repo::Repository;
use nest_protocol::entity::EntityId;

pub fn create(repo: &mut Repository, title: &str, body: Option<&str>) -> Result<()> {
    let id = repo.issue_create(title, body.unwrap_or(""))?;
    println!("Created issue: {}", id.as_str());
    Ok(())
}

pub fn list(repo: &Repository) -> Result<()> {
    let issues = repo.issue_list()?;
    if issues.is_empty() {
        println!("No issues.");
        return Ok(());
    }
    for issue in &issues {
        println!(
            "  #{} [{}] {} ({:?})",
            issue.number,
            issue.id.as_str(),
            issue.title.get(),
            issue.status.get(),
        );
    }
    Ok(())
}

pub fn show(repo: &Repository, id: &str) -> Result<()> {
    let issue = repo.issue_get(&EntityId::new(id))?;
    println!("Issue #{}: {}", issue.number, issue.title.get());
    println!("  Status: {:?}", issue.status.get());
    println!("  Priority: {:?}", issue.priority.get());
    println!("  Created by: {}", issue.created_by);
    println!("  Body: {}", issue.body.to_string());

    let labels: Vec<&String> = issue.labels.iter().collect();
    if !labels.is_empty() {
        println!("  Labels: {}", labels.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "));
    }

    let assignees: Vec<&String> = issue.assignees.iter().collect();
    if !assignees.is_empty() {
        println!("  Assignees: {}", assignees.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "));
    }

    let comments: Vec<_> = issue.comments.iter().collect();
    if !comments.is_empty() {
        println!("\n  Comments:");
        for comment in comments {
            println!("    {} wrote:", comment.author);
            println!("      {}", comment.body);
        }
    }

    Ok(())
}

pub fn close(repo: &mut Repository, id: &str) -> Result<()> {
    repo.issue_close(&EntityId::new(id))?;
    println!("Closed issue: {}", id);
    Ok(())
}

pub fn comment(repo: &mut Repository, id: &str, body: &str) -> Result<()> {
    repo.issue_add_comment(&EntityId::new(id), body)?;
    println!("Added comment to issue: {}", id);
    Ok(())
}
