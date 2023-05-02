//! Commit

use git2::StatusOptions;
use time::OffsetDateTime;

use crate::{error::Error, util::convert_git2_time, GitRepository};

/// A commit
#[derive(Debug)]
pub struct Commit {
    /// ID (hash)
    pub id: String,
    /// Date
    pub date: OffsetDateTime,
    /// Author name
    pub author_name: String,
    /// Author email
    pub author_email: String,
    /// Committer name
    pub committer_name: String,
    /// Committer email
    pub committer_email: String,
    /// Message
    pub message: String,
}

impl Commit {
    /// Returns the commit subject (1st line)
    pub fn subject(&self) -> String {
        if let Some(line) = self.message.lines().next() {
            return line.to_string();
        }
        unreachable!()
    }
}

impl<'repo> TryFrom<&'repo git2::Commit<'repo>> for Commit {
    type Error = Error;

    fn try_from(c: &'repo git2::Commit) -> Result<Self, Self::Error> {
        Ok(Commit {
            id: c.id().to_string(),
            date: convert_git2_time(c.time())?,
            author_name: c
                .author()
                .name()
                .ok_or(Error::msg("non UTF8 author name"))?
                .to_string(),
            author_email: c
                .author()
                .email()
                .ok_or(Error::msg("non UTF8 author email"))?
                .to_string(),
            committer_name: c
                .committer()
                .name()
                .ok_or(Error::msg("non UTF8 committer name"))?
                .to_string(),
            committer_email: c
                .committer()
                .email()
                .ok_or(Error::msg("non UTF8 committer email"))?
                .to_string(),
            message: c
                .message()
                .ok_or(Error::msg("non UTF8 message"))?
                .to_string(),
        })
    }
}

/// Returns the complete commit history
///
/// The returned list is ordered with the last commit first (revwalk order).
pub fn commit_log(repo: &GitRepository) -> Result<Vec<Commit>, Error> {
    // NB: an explanation can be found here (for Go)
    // https://stackoverflow.com/questions/37289674/how-to-run-git-log-commands-using-libgit2-in-go
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    // NB: revwalk starts with the last commit first
    let mut commits: Vec<_> = vec![];
    for oid_res in revwalk {
        match oid_res {
            Ok(oid) => {
                let obj = repo.find_object(oid, None).unwrap();
                //eprintln!("{:?}", obj);
                if let Some(c) = obj.as_commit() {
                    // eprintln!("{:#?}", c);
                    // NB: raw text values can be invalid if not UTF8
                    commits.push(c.try_into()?);
                }
            }
            Err(err) => return Err(Error::msg(format!("{err} (revwalk)").as_str())),
        }
    }
    Ok(commits)
}

/// Performs a commit to the head
pub fn commit_to_head(repo: &GitRepository, message: &str) -> Result<Commit, Error> {
    // check for nothing to commit
    let mut status_opts = StatusOptions::new();
    status_opts.show(git2::StatusShow::Index);
    let has_no_changes = repo.statuses(Some(&mut status_opts))?.is_empty();
    if has_no_changes {
        return Err(Error::msg("nothing to commit"));
    }

    // go
    let sig = repo.signature()?;
    let update_ref = Some("HEAD");
    let head = repo.head()?;
    let head_commit = head.peel_to_commit()?;

    let tree = {
        // NB: OK, here it is weird, the loaded index contains a bunch of stuff,
        // but i cannot see the staged changes. I just load the index and writes the tree
        // and it seems to work fine.
        let mut index = repo.index()?;
        let tree_id = index.write_tree()?;
        repo.find_tree(tree_id)?
    };
    let commit_id = repo.commit(update_ref, &sig, &sig, message, &tree, &[&head_commit])?;
    let commit_obj = repo.find_object(commit_id, None)?;
    let commit = commit_obj.as_commit().unwrap();
    commit.try_into()
}

#[cfg(test)]
mod tests {
    use crate::repo::discover_repo;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_commit_log() {
        let cwd = std::env::current_dir().unwrap();
        let repo = discover_repo(&cwd).unwrap();
        let commits = commit_log(&repo).unwrap();
        for c in commits {
            eprintln!("commit {}", c.id);
            eprintln!("Date: {}", c.date);
            eprintln!("Author: {} <{}>", c.author_name, c.author_email);
            eprintln!("Committer: {} <{}>", c.committer_name, c.committer_email);
            eprintln!("{}", c.message);
            eprintln!();
        }
    }

    #[test]
    fn test_commit_do() {
        let cwd = std::env::current_dir().unwrap();
        let _repo = discover_repo(&cwd).unwrap();
        // USE WITH CAUTION
        // commit_to_head(&repo, "test commit").unwrap();
    }
}
