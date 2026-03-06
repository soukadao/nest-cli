use anyhow::Result;
use nest_core::repo::Repository;
use nest_protocol::entity::EntityId;

pub fn create(repo: &mut Repository, title: &str, source: &str, target: &str) -> Result<()> {
    let id = repo.review_create(title, "", source, target)?;
    println!("Created review: {}", id.as_str());
    Ok(())
}

pub fn list(repo: &Repository) -> Result<()> {
    let reviews = repo.review_list()?;
    if reviews.is_empty() {
        println!("No reviews.");
        return Ok(());
    }
    for review in &reviews {
        println!(
            "  #{} [{}] {} ({:?}) {}->{}",
            review.number,
            review.id.as_str(),
            review.title.get(),
            review.status.get(),
            review.source_group.get(),
            review.target_group.get(),
        );
    }
    Ok(())
}

pub fn show(repo: &Repository, id: &str) -> Result<()> {
    let review = repo.review_get(&EntityId::new(id))?;
    println!("Review #{}: {}", review.number, review.title.get());
    println!("  Status:  {:?}", review.status.get());
    println!("  Source:  {}", review.source_group.get());
    println!("  Target:  {}", review.target_group.get());
    println!("  Author:  {}", review.author);

    let reviewers: Vec<&String> = review.reviewers.iter().collect();
    if !reviewers.is_empty() {
        println!(
            "  Reviewers: {}",
            reviewers
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    let approvals: Vec<_> = review.approvals.iter().collect();
    if !approvals.is_empty() {
        println!("  Approvals:");
        for a in &approvals {
            println!("    - {}", a.reviewer);
        }
    }

    let desc = review.description.to_string();
    if !desc.is_empty() {
        println!("\n  Description:\n    {}", desc);
    }

    let comments: Vec<_> = review.comments.iter().collect();
    if !comments.is_empty() {
        println!("\n  Comments:");
        for c in comments {
            if let Some(ref path) = c.file_path {
                print!("    [{}] ", path);
            } else {
                print!("    ");
            }
            println!("{} wrote:", c.author);
            println!("      {}", c.body);
        }
    }

    Ok(())
}

pub fn approve(repo: &mut Repository, id: &str) -> Result<()> {
    repo.review_approve(&EntityId::new(id))?;
    println!("Approved review: {}", id);
    Ok(())
}

pub fn close(repo: &mut Repository, id: &str) -> Result<()> {
    repo.review_close(&EntityId::new(id))?;
    println!("Closed review: {}", id);
    Ok(())
}

pub fn comment(repo: &mut Repository, id: &str, body: &str) -> Result<()> {
    repo.review_comment(&EntityId::new(id), body, None, None)?;
    println!("Added comment to review: {}", id);
    Ok(())
}

pub fn merge(repo: &mut Repository, id: &str) -> Result<()> {
    repo.review_merge(&EntityId::new(id))?;
    println!("Merged review: {}", id);
    Ok(())
}
