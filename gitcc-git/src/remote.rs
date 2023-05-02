//! Remote

use crate::{error::Error, GitRepository};

/// Performs a push
pub fn push_branch(_repo: &GitRepository) -> Result<(), Error> {
    // find the remote => git2::Remote
    // push the branch => remote.push(refspecs)
    // refspecs maop a branch in the local repo to the remote repo
    todo!()
}
