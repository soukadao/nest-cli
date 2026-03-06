use anyhow::Result;
use nest_core::repo::Repository;
use nest_protocol::api::DiffLineKind;

pub fn run(repo: &Repository, from: Option<&str>, to: Option<&str>) -> Result<()> {
    let diff = repo.snapshot_diff(from, to)?;

    if diff.files.is_empty() {
        println!("No differences.");
        return Ok(());
    }

    for file_diff in &diff.files {
        println!("--- a/{}", file_diff.path);
        println!("+++ b/{}", file_diff.path);
        println!("Status: {:?}", file_diff.status);

        for hunk in &file_diff.hunks {
            println!(
                "@@ -{},{} +{},{} @@",
                hunk.old_start, hunk.old_count, hunk.new_start, hunk.new_count
            );
            for line in &hunk.lines {
                match line.kind {
                    DiffLineKind::Context => println!(" {}", line.content),
                    DiffLineKind::Add => println!("+{}", line.content),
                    DiffLineKind::Delete => println!("-{}", line.content),
                }
            }
        }
        println!();
    }

    Ok(())
}
