//! Git commands

use std::{
    cmp::max,
    collections::{BTreeMap, HashMap},
    fmt::Display,
    path::Path,
};

use gitcc_convco::{ConvcoMessage, DEFAULT_CONVCO_INCR_MINOR_TYPES, DEFAULT_CONVCO_TYPES};
use gitcc_git::discover_repo;
use semver::Version;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

pub use gitcc_git::StatusShow;

use crate::{Config, Error};

/// Commits configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct CommitConfig {
    /// Valid commit types (key + description)
    pub types: BTreeMap<String, String>,
}

impl Default for CommitConfig {
    fn default() -> Self {
        Self {
            types: DEFAULT_CONVCO_TYPES
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }
    }
}

/// Versioning configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct VersioningConfig {
    /// List of commit types which increment the minor version
    ///
    /// Those are enhancements (vs fixes or cosmetic changes)
    pub types_incr_minor: Vec<String>,
}

impl Default for VersioningConfig {
    fn default() -> Self {
        Self {
            types_incr_minor: DEFAULT_CONVCO_INCR_MINOR_TYPES
                .map(|s| s.to_string())
                .to_vec(),
        }
    }
}

/// A commit
///
/// This commit object extends the std commit with:
/// - its tag
/// - the parsed conventional message
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
    /// Raw message
    pub raw_message: String,
    /// Parsed convco message (None if not a conventional message)
    pub conv_message: Option<ConvcoMessage>,
    /// Tag object
    pub tag: Option<gitcc_git::Tag>,
    /// Version to which the commit belongs (None = unreleased)
    pub version_tag: Option<gitcc_git::Tag>,
}

impl Commit {
    /// Returns the short id (7 chars)
    pub fn short_id(&self) -> String {
        let mut short_id = self.id.clone();
        short_id.truncate(7);
        short_id
    }

    /// Returns the commit subject (1st line)
    pub fn subject(&self) -> String {
        if let Some(line) = self.raw_message.lines().next() {
            return line.to_string();
        }
        unreachable!()
    }
}

/// The semver version increment
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VersionIncr {
    None,
    Patch,
    Minor,
    Major,
}

impl Display for VersionIncr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VersionIncr::None => write!(f, "na"),
            VersionIncr::Patch => write!(f, "patch"),
            VersionIncr::Minor => write!(f, "minor"),
            VersionIncr::Major => write!(f, "major"),
        }
    }
}

impl VersionIncr {
    /// Applies a version increment to a version
    fn apply(&self, version: &Option<Version>) -> Version {
        if let Some(v) = version {
            if v.major == 0 {
                match self {
                    VersionIncr::None => v.clone(),
                    VersionIncr::Patch => Version::new(0, v.minor + 1, 0),
                    VersionIncr::Minor => Version::new(0, v.minor + 1, 0),
                    VersionIncr::Major => Version::new(0, v.minor + 1, 0),
                }
            } else {
                match self {
                    VersionIncr::None => v.clone(),
                    VersionIncr::Patch => Version::new(v.major, v.minor, v.patch + 1),
                    VersionIncr::Minor => Version::new(v.major, v.minor + 1, 0),
                    VersionIncr::Major => Version::new(v.major + 1, 0, 0),
                }
            }
        } else {
            Version::new(0, 1, 0)
        }
    }
}

/// Extension trait for conventional messages
pub trait ConvcoMessageExt {
    /// Determines the kin of version increment for a conventional message
    fn version_incr_kind(&self, cfg: &VersioningConfig) -> VersionIncr;
}

impl ConvcoMessageExt for ConvcoMessage {
    fn version_incr_kind(&self, cfg: &VersioningConfig) -> VersionIncr {
        if self.is_breaking_change() {
            return VersionIncr::Major;
        }

        if cfg.types_incr_minor.contains(&self.r#type) {
            return VersionIncr::Minor;
        }
        VersionIncr::Patch
    }
}

/// Commit history
#[derive(Debug)]
pub struct CommitHistory {
    /// Commits
    ///
    /// The list is ordered with the last commit first
    pub commits: Vec<Commit>,
    /// Current version
    pub curr_version: Option<Version>,
    /// Next version (unreleased)
    pub next_version: Version,
}

impl CommitHistory {
    pub fn next_version_str(&self) -> String {
        format!("v{}", self.next_version)
    }
}

/// Checks if the repo has unstaged or untracked files
pub fn git_status(
    cwd: &Path,
    show: StatusShow,
) -> Result<BTreeMap<String, gitcc_git::Status>, Error> {
    let repo = discover_repo(cwd)?;
    let files = gitcc_git::repo_status(&repo, show)?;
    Ok(files)
}

/// Adds all changes to the index
pub fn git_add_all(cwd: &Path) -> Result<(), Error> {
    let repo = discover_repo(cwd)?;
    gitcc_git::add_all(&repo)?;
    Ok(())
}

/// Returns the history of all commits
pub fn commit_history(cwd: &Path, cfg: &Config) -> Result<CommitHistory, Error> {
    let repo = gitcc_git::discover_repo(cwd)?;
    let git_commits = gitcc_git::commit_log(&repo)?;
    let map_commit_to_tag: HashMap<_, _> = gitcc_git::get_tag_refs(&repo)?
        .into_iter()
        .map(|t| (t.commit_id.clone(), t))
        .collect();

    let mut commits = Vec::new();
    let mut curr_version: Option<Version> = None; // current version
    let mut latest_version_tag: Option<gitcc_git::Tag> = None;
    let mut unreleased_incr_kind = VersionIncr::None; // type of increment for the next version
    let mut is_commit_released = false;
    for c in git_commits {
        // NB: this loop is with the last commit first, so we walk towards the 1st commit
        let conv_message = match c.message.parse::<ConvcoMessage>() {
            Ok(m) => {
                if !cfg.commit.types.contains_key(&m.r#type) {
                    log::debug!("commit {} has an invalid type: {}", c.id, m.r#type);
                }
                Some(m)
            }
            Err(err) => {
                log::debug!(
                    "commit {} does not follow the conventional commit format: {}",
                    c.id,
                    err
                );
                None
            }
        };

        let tag = map_commit_to_tag.get(&c.id).cloned();

        // if an annotated tag is found, set the commit version
        let mut has_annotated_tag = false;
        if let Some(tag) = &tag {
            if tag.is_annotated() {
                has_annotated_tag = true
            }
        }
        if has_annotated_tag {
            let tag = tag.clone().unwrap();
            let tag_name = tag.name.trim();
            let tag_version = tag_name.strip_prefix('v').unwrap_or(tag_name);
            match tag_version.parse::<Version>() {
                Ok(v) => {
                    // eprintln!(" => version: {}", v);
                    latest_version_tag = Some(tag);
                    if curr_version.is_none() {
                        curr_version = Some(v);
                    }
                    is_commit_released = true;
                }
                Err(err) => {
                    log::debug!(
                        "commit {} has tag {} which is not a semver version: {}",
                        c.id,
                        tag.name,
                        err
                    );
                }
            }
        }

        // find how to increment the next version for unreleaed commits
        if !is_commit_released {
            if let Some(m) = &conv_message {
                let commit_incr_kind = m.version_incr_kind(&cfg.version);
                unreleased_incr_kind = max(unreleased_incr_kind, commit_incr_kind);
            } else {
                unreleased_incr_kind = max(unreleased_incr_kind, VersionIncr::Patch);
            }
        }

        commits.push(Commit {
            id: c.id,
            date: c.date,
            author_name: c.author_name,
            author_email: c.author_email,
            committer_name: c.committer_name,
            committer_email: c.committer_email,
            raw_message: c.message,
            conv_message,
            tag,
            version_tag: latest_version_tag.clone(),
        });
    }

    let next_version = unreleased_incr_kind.apply(&curr_version);

    Ok(CommitHistory {
        commits,
        curr_version,
        next_version,
    })
}

/// Commits the changes to git
pub fn commit_changes(cwd: &Path, message: &str) -> Result<gitcc_git::Commit, Error> {
    let repo = gitcc_git::discover_repo(cwd)?;
    Ok(gitcc_git::commit_to_head(&repo, message)?)
}

#[cfg(test)]
mod tests {
    use time::macros::format_description;

    use super::*;

    #[test]
    fn test_history() {
        let cwd = std::env::current_dir().unwrap();
        let cfg = Config::load_from_fs(&cwd).unwrap().unwrap_or_default();
        let history = commit_history(&cwd, &cfg).unwrap();
        for c in &history.commits {
            eprintln!(
                "{}: {} | {} | {} {}",
                c.date
                    .format(format_description!("[year]-[month]-[day]"))
                    .unwrap(),
                c.conv_message
                    .as_ref()
                    .map(|m| m.r#type.clone())
                    .unwrap_or("--".to_string()),
                if c.version_tag.is_none() {
                    c.conv_message
                        .as_ref()
                        .map(|m| m.version_incr_kind(&cfg.version).to_string())
                        .unwrap_or("--".to_string())
                } else {
                    "--".to_string()
                },
                c.version_tag
                    .as_ref()
                    .map(|t| t.name.to_string())
                    .unwrap_or("unreleased".to_string()),
                if let Some(tag) = &c.tag {
                    format!("<- {}", &tag.name)
                } else {
                    "".to_string()
                }
            );
        }
        eprintln!();
        eprintln!(
            "current version: {}",
            history
                .curr_version
                .map(|v| v.to_string())
                .unwrap_or("unreleased".to_string())
        );
        eprintln!("next version: {}", history.next_version);
        eprintln!();
    }
}
