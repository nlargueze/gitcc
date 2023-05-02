//! Status

use std::collections::BTreeMap;

use git2::StatusOptions;

use crate::{error::Error, repo::GitRepository};

/// Status of a file
pub type Status = git2::Status;

/// Status show option
pub type StatusShow = git2::StatusShow;

/// Checks if the repo is clean, ie has no uncommited and untracked files
pub fn repo_status(
    repo: &GitRepository,
    show: StatusShow,
) -> Result<BTreeMap<String, git2::Status>, Error> {
    if repo.is_bare() {
        return Err(Error::msg("cannot report status on bare repository"));
    }

    let mut opts = StatusOptions::new();
    opts.show(show).include_untracked(true);
    let entries: BTreeMap<String, git2::Status> = repo
        .statuses(Some(&mut opts))?
        .into_iter()
        .map(|e| {
            (
                e.path().map(|p| p.to_string()).unwrap_or_default(),
                e.status(),
            )
        })
        .collect();

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
        let dirty_files = repo_status(&repo, StatusShow::IndexAndWorkdir).unwrap();
        for (file, status) in dirty_files {
            eprintln!("{:?}: {:?}", file, status);
        }
    }
}
