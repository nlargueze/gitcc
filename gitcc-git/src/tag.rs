//! Tags

use time::OffsetDateTime;

use crate::{error::Error, util::convert_git2_time, GitRepository};

/// A git tag
#[derive(Debug, Clone)]
pub struct Tag {
    /// ID (hash)
    pub id: String,
    /// Date
    pub date: OffsetDateTime,
    /// Name (short)
    pub name: String,
    /// Full name
    pub name_full: String,
    /// Tag message - None if lightweight tag
    pub message: Option<String>,
    /// Commit ID (hash)
    pub commit_id: String,
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Tag {
    /// Checks if the tag is annotated
    pub fn is_annotated(&self) -> bool {
        self.message.is_some()
    }
}

/// Retrieves all the repo tags (lightweight and annotated)
pub fn list_tags(repo: &GitRepository) -> Result<Vec<String>, Error> {
    let tags = repo.tag_names(None)?;
    let tags = tags
        .into_iter()
        .filter_map(|t| t.map(|s| s.to_string()))
        .collect();
    Ok(tags)
}

/// Retrieves all the repo tag references
///
/// This method looks for all references and finds the tags.
pub fn get_tag_refs(repo: &GitRepository) -> Result<Vec<Tag>, Error> {
    let refs = repo.references()?;

    let mut tags = vec![];
    for res in refs {
        let rf = res?;

        // resolve symbolic tags
        let rf = rf.resolve()?;

        // extract data
        let id = rf.target().unwrap().to_string();
        let full_name = rf.name().unwrap_or("__invalid__").to_string();
        let name = rf.shorthand().unwrap_or("__invalid__").to_string();

        // a tag starts with 'refs/tags'
        // if it is an annotated tag, it is possible to peel back to a Tag
        // eprintln!("ref: {full_name}");
        if !full_name.starts_with("refs/tags/") {
            // eprintln!("not a tag => skipped");
            continue;
        }
        // peel to tag to check if the ref is a tag
        // NB: lightweight tags do not have ref of their own
        let tag = rf.peel_to_tag().ok();
        let tag_message = tag.map(|t| t.message().unwrap_or("__invalid__").trim().to_string());

        // peel to find the commit
        // NB: a tag always points to a commit (itself for a lightweight tag)
        let commit = rf.peel_to_commit()?;
        let commit_id = commit.id().to_string();
        let date = convert_git2_time(commit.time())?;

        tags.push({
            Tag {
                id,
                date,
                name,
                name_full: full_name,
                message: tag_message,
                commit_id,
            }
        })
    }

    Ok(tags)
}

#[cfg(test)]
mod tests {
    use crate::repo::discover_repo;

    use super::*;

    #[test]
    fn test_tags_simple() {
        let cwd = std::env::current_dir().unwrap();
        let repo = discover_repo(&cwd).unwrap();
        let tags = list_tags(&repo).unwrap();
        for tag in tags {
            eprintln!("{tag}")
        }
    }

    #[test]
    fn test_tags_refs() {
        let cwd = std::env::current_dir().unwrap();
        let repo = discover_repo(&cwd).unwrap();
        let tags = get_tag_refs(&repo).unwrap();
        for tag in tags {
            eprintln!("{}:  {} ({})", tag.id, tag.name, tag.commit_id)
        }
    }
}
