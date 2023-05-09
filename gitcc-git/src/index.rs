//! Index

use git2::IndexAddOption;

use crate::{Error, GitRepository};

/// Runs `git add --all`
pub fn add_all(repo: &GitRepository) -> Result<(), Error> {
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
    index.write()?;
    Ok(())
}
