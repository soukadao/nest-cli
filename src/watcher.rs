use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Result;
use nest_core::repo::Repository;

/// File watcher that detects changes and generates CRDT operations.
pub struct FileWatcher {
    root: PathBuf,
    file_cache: HashMap<PathBuf, String>,
    ignore_patterns: Vec<String>,
}

impl FileWatcher {
    pub fn new(root: &Path) -> Self {
        FileWatcher {
            root: root.to_path_buf(),
            file_cache: HashMap::new(),
            ignore_patterns: vec![
                ".nest".to_string(),
                ".git".to_string(),
                "target".to_string(),
                "node_modules".to_string(),
            ],
        }
    }

    /// Scan all files and record initial state.
    pub fn scan_initial(&mut self) -> Result<()> {
        self.scan_dir(&self.root.clone())
    }

    /// Check for changes and return the list of changed files.
    pub fn detect_changes(&mut self) -> Result<Vec<FileChange>> {
        let mut changes = Vec::new();
        let mut current_files = HashMap::new();

        self.collect_files(&self.root.clone(), &mut current_files)?;

        // Detect modifications and additions
        for (path, content) in &current_files {
            match self.file_cache.get(path) {
                Some(old_content) if old_content != content => {
                    changes.push(FileChange {
                        path: path.clone(),
                        old_content: Some(old_content.clone()),
                        new_content: Some(content.clone()),
                        kind: ChangeKind::Modified,
                    });
                }
                None => {
                    changes.push(FileChange {
                        path: path.clone(),
                        old_content: None,
                        new_content: Some(content.clone()),
                        kind: ChangeKind::Added,
                    });
                }
                _ => {}
            }
        }

        // Detect deletions
        for path in self.file_cache.keys() {
            if !current_files.contains_key(path) {
                changes.push(FileChange {
                    path: path.clone(),
                    old_content: self.file_cache.get(path).cloned(),
                    new_content: None,
                    kind: ChangeKind::Deleted,
                });
            }
        }

        // Update cache
        self.file_cache = current_files;

        Ok(changes)
    }

    /// Record changes into the repository's active stream.
    pub fn record_changes(repo: &mut Repository, changes: &[FileChange]) -> Result<usize> {
        let mut op_count = 0;
        for change in changes {
            let rel_path = change
                .path
                .strip_prefix(repo.root())
                .unwrap_or(&change.path);
            let rel_str = rel_path.to_string_lossy().to_string();

            match change.kind {
                ChangeKind::Added | ChangeKind::Modified => {
                    let old = change.old_content.as_deref().unwrap_or("");
                    let new = change.new_content.as_deref().unwrap_or("");
                    let ops = repo.record_file_change(&rel_str, old, new)?;
                    op_count += ops.len();
                }
                ChangeKind::Deleted => {
                    // Record as empty content
                    let old = change.old_content.as_deref().unwrap_or("");
                    let ops = repo.record_file_change(&rel_str, old, "")?;
                    op_count += ops.len();
                }
            }
        }
        Ok(op_count)
    }

    fn scan_dir(&mut self, dir: &Path) -> Result<()> {
        let mut files = HashMap::new();
        self.collect_files(dir, &mut files)?;
        self.file_cache = files;
        Ok(())
    }

    fn collect_files(&self, dir: &Path, files: &mut HashMap<PathBuf, String>) -> Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            if self.ignore_patterns.iter().any(|p| name == *p) {
                continue;
            }

            if path.is_dir() {
                self.collect_files(&path, files)?;
            } else if path.is_file() {
                // Only track text files
                if let Ok(content) = std::fs::read_to_string(&path) {
                    files.insert(path, content);
                }
            }
        }

        Ok(())
    }
}

pub struct FileChange {
    pub path: PathBuf,
    pub old_content: Option<String>,
    pub new_content: Option<String>,
    pub kind: ChangeKind,
}

pub enum ChangeKind {
    Added,
    Modified,
    Deleted,
}
