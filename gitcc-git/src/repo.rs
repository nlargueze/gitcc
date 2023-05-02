//! Repo management

use std::path::Path;

use crate::error::Error;

/// A simple alias for git2::Repository
pub type GitRepository = git2::Repository;

/// Discovers the repo at or above the provided path
pub fn discover_repo(p: &Path) -> Result<GitRepository, Error> {
    Ok(git2::Repository::discover(p)?)
}
