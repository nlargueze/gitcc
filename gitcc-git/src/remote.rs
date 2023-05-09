//! Remote

use crate::{error::Error, GitRepository};

/// Performs a push
pub fn push_to_remote(repo: &GitRepository, remote_name: &str) -> Result<(), Error> {
    let mut remote = repo.find_remote(remote_name)?;
    // NB: if no refspecs is passed, the configured refspecs are passed
    let refspecs: [String; 0] = [];
    Ok(remote.push::<_>(&refspecs, None)?)
}
