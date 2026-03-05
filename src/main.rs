mod cmd;
mod sync_client;
mod watcher;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "nest", version, about = "Next-generation version control system")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Nest repository
    Init {
        /// Path to initialize (default: current directory)
        #[arg(default_value = ".")]
        path: String,
        /// User name
        #[arg(short, long, default_value = "anonymous")]
        user: String,
    },
    /// Show repository status
    Status,
    /// Show snapshot history
    Log,
    /// Show diff between snapshots
    Diff {
        /// Source snapshot ID
        from: Option<String>,
        /// Target snapshot ID
        to: Option<String>,
    },
    /// Manage branches
    Branch {
        #[command(subcommand)]
        action: BranchAction,
    },
    /// Record file changes
    Record,
    /// Manage snapshots
    Snapshot {
        #[command(subcommand)]
        action: SnapshotAction,
    },
    /// Manage issues
    Issue {
        #[command(subcommand)]
        action: IssueAction,
    },
    /// Manage reviews
    Review {
        #[command(subcommand)]
        action: ReviewAction,
    },
    /// Manage documents
    Doc {
        #[command(subcommand)]
        action: DocAction,
    },
    /// Manage remotes
    Remote {
        #[command(subcommand)]
        action: RemoteAction,
    },
    /// Sync with remote
    Sync {
        /// Remote name
        remote: Option<String>,
    },
}

#[derive(Subcommand)]
enum BranchAction {
    /// List branches
    List,
    /// Create a new branch
    Create { name: String },
    /// Switch to a branch
    Switch { name: String },
}

#[derive(Subcommand)]
enum SnapshotAction {
    /// Create a snapshot
    Create {
        /// Snapshot message
        #[arg(short, long)]
        message: String,
    },
    /// List snapshots
    List,
}

#[derive(Subcommand)]
enum IssueAction {
    /// Create an issue
    Create {
        title: String,
        #[arg(short, long)]
        body: Option<String>,
    },
    /// List issues
    List,
    /// Show issue details
    Show { id: String },
    /// Close an issue
    Close { id: String },
    /// Add a comment
    Comment { id: String, body: String },
}

#[derive(Subcommand)]
enum ReviewAction {
    /// Create a review
    Create {
        title: String,
        #[arg(long)]
        source: String,
        #[arg(long)]
        target: String,
    },
    /// List reviews
    List,
}

#[derive(Subcommand)]
enum DocAction {
    /// Create a document
    Create {
        title: String,
        /// Document body (if omitted, opens $EDITOR)
        #[arg(short, long)]
        body: Option<String>,
    },
    /// List documents
    List,
    /// Show a document
    Show { id: String },
    /// Edit a document with $EDITOR
    Edit { id: String },
}

#[derive(Subcommand)]
enum RemoteAction {
    /// Add a remote
    Add { name: String, url: String },
    /// List remotes
    List,
    /// Remove a remote
    Remove { name: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { path, user } => {
            let path =
                PathBuf::from(&path).canonicalize().unwrap_or_else(|_| PathBuf::from(&path));
            cmd::init::run(&path, &user)?;
        }
        Commands::Status => {
            let repo = open_repo()?;
            let status = repo.status()?;
            println!("On branch: {}", status.branch);
            println!("User:      {}", status.user);
        }
        Commands::Log => {
            let repo = open_repo()?;
            cmd::log::run(&repo)?;
        }
        Commands::Diff { from, to } => {
            let repo = open_repo()?;
            cmd::diff::run(&repo, from.as_deref(), to.as_deref())?;
        }
        Commands::Branch { action } => {
            let mut repo = open_repo()?;
            match action {
                BranchAction::List => cmd::branch::list(&repo)?,
                BranchAction::Create { name } => cmd::branch::create(&mut repo, &name)?,
                BranchAction::Switch { name } => cmd::branch::switch(&mut repo, &name)?,
            }
        }
        Commands::Record => {
            let mut repo = open_repo()?;
            let mut watcher = watcher::FileWatcher::new(repo.root());
            watcher.scan_initial()?;
            let changes = watcher.detect_changes()?;
            if changes.is_empty() {
                println!("No changes detected.");
            } else {
                let count = watcher::FileWatcher::record_changes(&mut repo, &changes)?;
                println!(
                    "Recorded {} operations from {} file(s).",
                    count,
                    changes.len()
                );
            }
        }
        Commands::Snapshot { action } => {
            let mut repo = open_repo()?;
            match action {
                SnapshotAction::Create { message } => {
                    cmd::snapshot::create(&mut repo, &message)?;
                }
                SnapshotAction::List => cmd::snapshot::list(&repo)?,
            }
        }
        Commands::Issue { action } => {
            let mut repo = open_repo()?;
            match action {
                IssueAction::Create { title, body } => {
                    cmd::issue::create(&mut repo, &title, body.as_deref())?;
                }
                IssueAction::List => cmd::issue::list(&repo)?,
                IssueAction::Show { id } => cmd::issue::show(&repo, &id)?,
                IssueAction::Close { id } => cmd::issue::close(&mut repo, &id)?,
                IssueAction::Comment { id, body } => {
                    cmd::issue::comment(&mut repo, &id, &body)?;
                }
            }
        }
        Commands::Review { action } => {
            let mut repo = open_repo()?;
            match action {
                ReviewAction::Create {
                    title,
                    source,
                    target,
                } => cmd::review::create(&mut repo, &title, &source, &target)?,
                ReviewAction::List => cmd::review::list(&repo)?,
            }
        }
        Commands::Doc { action } => {
            let mut repo = open_repo()?;
            match action {
                DocAction::Create { title, body } => {
                    cmd::doc::create(&mut repo, &title, body.as_deref())?;
                }
                DocAction::List => cmd::doc::list(&repo)?,
                DocAction::Show { id } => cmd::doc::show(&repo, &id)?,
                DocAction::Edit { id } => cmd::doc::edit(&mut repo, &id)?,
            }
        }
        Commands::Remote { action } => {
            let mut repo = open_repo()?;
            match action {
                RemoteAction::Add { name, url } => cmd::remote::add(&mut repo, &name, &url)?,
                RemoteAction::List => cmd::remote::list(&repo)?,
                RemoteAction::Remove { name } => cmd::remote::remove(&mut repo, &name)?,
            }
        }
        Commands::Sync { remote } => {
            let mut repo = open_repo()?;
            cmd::sync::run(&mut repo, remote.as_deref()).await?;
        }
    }

    Ok(())
}

fn open_repo() -> Result<nest_core::repo::Repository> {
    let cwd = std::env::current_dir()?;
    let mut path = cwd.as_path();
    loop {
        if path.join(".nest").exists() {
            return nest_core::repo::Repository::open(path);
        }
        match path.parent() {
            Some(parent) => path = parent,
            None => {
                anyhow::bail!("Not a Nest repository (or any parent directory)");
            }
        }
    }
}
