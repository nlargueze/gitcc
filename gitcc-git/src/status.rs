//! Status

use std::fmt::Display;

use git2::StatusOptions;

use crate::{error::Error, repo::GitRepository};

/// Represents a file (or dir) with its git status
#[derive(Debug, Clone)]
pub struct GitFileStatus {
    /// Path to the file (None if the name is not UTF8)
    pub path: Option<String>,
    /// Git status
    pub status: git2::Status,
}

impl Display for GitFileStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {:?}",
            self.path.clone().unwrap_or("__invalid__".to_string()),
            self.status
        )
    }
}

/// Checks if the repo is clean, ie has no uncommited and untracked files
pub fn repo_status(repo: &GitRepository) -> Result<Vec<GitFileStatus>, Error> {
    if repo.is_bare() {
        return Err(Error::msg("cannot report status on bare repository"));
    }

    let mut opts = StatusOptions::new();
    opts.show(git2::StatusShow::IndexAndWorkdir)
        .include_untracked(true);
    let statuses = repo.statuses(Some(&mut opts))?;
    let mut entries: Vec<_> = statuses
        .into_iter()
        .map(|e| GitFileStatus {
            path: e.path().map(|p| p.to_string()),
            status: e.status(),
        })
        .collect();

    entries.sort_by(|f1, f2| {
        // sort by status
        f1.status.partial_cmp(&f2.status).unwrap()
    });

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use crate::repo::discover_repo;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_repo_status() {
        let cwd = std::env::current_dir().unwrap();
        let repo = discover_repo(&cwd).unwrap();
        let dirty_files = repo_status(&repo).unwrap();
        for f in dirty_files {
            eprintln!("{:?}: {:?}", f.path, f.status);
        }
    }
}
