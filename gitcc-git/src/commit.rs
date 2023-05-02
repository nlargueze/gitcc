//! Commit

use time::{OffsetDateTime, UtcOffset};

use crate::{error::Error, GitRepository};

/// A commit
#[derive(Debug)]
pub struct GitCommit {
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

impl GitCommit {
    /// Returns the commit subject (1st line)
    pub fn subject(&self) -> String {
        if let Some(line) = self.message.lines().next() {
            return line.to_string();
        }
        unreachable!()
    }
}

/// Returns the complete commit history
///
/// The returned list is ordered with the last commit first (revwalk order).
pub fn commit_history(repo: &GitRepository) -> Result<Vec<GitCommit>, Error> {
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
                    commits.push(GitCommit {
                        id: c.id().to_string(),
                        date: convert_git2_time(c.time())?,
                        author_name: c.author().name().unwrap_or("__invalid__").to_string(),
                        author_email: c.author().email().unwrap_or("__invalid__").to_string(),
                        committer_name: c.committer().name().unwrap_or("__invalid__").to_string(),
                        committer_email: c.committer().email().unwrap_or("__invalid__").to_string(),
                        message: c.message().unwrap_or("__invalid__").to_string(),
                    });
                }
            }
            Err(err) => return Err(Error::msg(format!("{err} (revwalk)").as_str())),
        }
    }
    Ok(commits)
}

/// Performs a commit
pub fn repo_push(repo: &GitRepository, _message: &str) -> Result<(), Error> {
    if repo.is_bare() {
        return Err(Error::msg("cannot commit on a bare repository"));
    }

    // let oid = repo.commit(Some("HEAD"), author, committer, message, tree, parents)?;
    todo!();
}

/// Converts a [git2::Time] to [OffsetDateTime]
fn convert_git2_time(time: git2::Time) -> Result<OffsetDateTime, Error> {
    let time_secs_unix = time.seconds();
    let mut dt = OffsetDateTime::from_unix_timestamp(time_secs_unix)
        .map_err(|err| Error::msg(err.to_string().as_str()))?;

    let time_tz_mins = time.offset_minutes();
    let offset = UtcOffset::from_whole_seconds(60 * time_tz_mins)
        .map_err(|err| Error::msg(err.to_string().as_str()))?;

    dt = dt.to_offset(offset);
    Ok(dt)
}

#[cfg(test)]
mod tests {
    use crate::repo::discover_repo;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_commit_history() {
        let cwd = std::env::current_dir().unwrap();
        let repo = discover_repo(&cwd).unwrap();
        let commits = commit_history(&repo).unwrap();
        for c in commits {
            eprintln!("commit {}", c.id);
            eprintln!("Date: {}", c.date);
            eprintln!("Author: {} <{}>", c.author_name, c.author_email);
            eprintln!("Committer: {} <{}>", c.committer_name, c.committer_email);
            eprintln!("{}", c.message);
            eprintln!();
        }
    }
}
