//! Commits

use std::{collections::BTreeMap, fmt::Display, io::BufRead, str::FromStr};

use anyhow::bail;
use colored::Colorize;
use regex::Regex;
use semver::{BuildMetadata, Prerelease, Version};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{git, util::StringExt};

/// Commits configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitsConfig {
    /// Commit types incrementing the minor part of the version
    pub increment_minor: Vec<String>,
    /// Valid commit types (key + description)
    pub types: BTreeMap<String, String>,
}

impl Default for CommitsConfig {
    fn default() -> Self {
        let mut types = BTreeMap::new();
        types.insert("feat".to_string(), "New features".to_string());
        types.insert("fix".to_string(), "Bug fixes".to_string());
        types.insert("docs".to_string(), "Documentation".to_string());
        types.insert("style".to_string(), "Code styling".to_string());
        types.insert("refactor".to_string(), "Code refactoring".to_string());
        types.insert("perf".to_string(), "Performance Improvements".to_string());
        types.insert("test".to_string(), "Testing".to_string());
        types.insert("build".to_string(), "Build system".to_string());
        types.insert("ci".to_string(), "Continuous Integration".to_string());
        types.insert("cd".to_string(), "Continuous Delivery".to_string());
        types.insert("chore".to_string(), "Other changes".to_string());

        let increment_minor = vec!["feat".to_string()];

        Self {
            types,
            increment_minor,
        }
    }
}

/// Conventional commit message
#[derive(Debug, Default, Clone)]
pub struct ConvCommitMessage {
    /// Commit type
    pub r#type: String,
    /// Commit scope
    pub scope: Option<String>,
    /// Commit subject
    pub subject: String,
    /// Commit body
    pub body: Option<String>,
    /// Breaking change
    pub breaking_change: Option<String>,
    /// Closed issues
    pub closed_issues: Option<Vec<u32>>,
}

impl Display for ConvCommitMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{}: {}",
            self.r#type,
            self.scope
                .as_ref()
                .map(|s| format!("({s})"))
                .unwrap_or_default(),
            self.breaking_change
                .as_ref()
                .map(|_| "!")
                .unwrap_or_default(),
            self.subject
        )?;

        // Body
        if let Some(b) = &self.body {
            writeln!(f)?;
            writeln!(f, "{b}")?;
        }

        // Breaking change
        if let Some(msg) = &self.breaking_change {
            writeln!(f)?;
            writeln!(f, "BREAKING_ CHANGE: {msg}")?;
        }

        // Closed issues
        if let Some(issues) = &self.closed_issues {
            writeln!(
                f,
                "Closes: {}",
                issues
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            )?;
        }

        Ok(())
    }
}

impl ConvCommitMessage {
    /// Parses a string into a conventional commit message
    pub fn parse(s: &str, config: &CommitsConfig) -> anyhow::Result<Self> {
        let mut r#type = String::new();
        let mut scope: Option<String> = None;
        let mut subject = String::new();
        let mut body: Option<String> = None;
        let mut breaking_change: Option<String> = None;
        let mut closed_issues: Option<Vec<u32>> = None;

        // cf. https://2fd.github.io/rust-regex-playground
        let regex_prefix =
            Regex::new(r"(?P<type>[[:word:]]+)(?P<scope>(\([0-9A-Za-z_\s]+\))?)(?P<breaking>!?)")
                .expect("Invalid regex");

        #[derive(Debug, PartialEq)]
        enum Section {
            Subject,
            Body,
            FooterBreakingChange,
            FooterCloseIssue,
        }
        let mut prev_section = Section::Subject;

        for (i, line_res) in s.as_bytes().lines().enumerate() {
            let line = line_res?;

            // >> 1st line
            if i == 0 {
                // eprintln!("|> subject line");
                let parts: Vec<_> = line.splitn(2, ':').collect();
                if parts.len() != 2 {
                    bail!("Conventional commit missing ':' separator");
                }
                // parse the prefix
                let prefix = parts[0].trim().to_string();
                if let Some(capts) = regex_prefix.captures(&prefix) {
                    // get type
                    r#type = match capts.name("type") {
                        Some(m) => {
                            let t = m.as_str();
                            if !config.types.keys().cloned().any(|x| x == *t) {
                                bail!("Invalid conventional commit type '{}'", t);
                            }
                            t.to_string()
                        }
                        None => {
                            bail!("Missing conventional commit type in '{line}'");
                        }
                    };
                    // get scope
                    scope = match capts.name("scope") {
                        Some(m) => {
                            let s = m.as_str().trim().to_string();
                            if s.is_empty() {
                                None
                            } else {
                                let s = s.trim_matches('(').trim_matches(')');
                                // check lowercase
                                if !s.starts_with_lowercase() {
                                    bail!("Invalid commit: scope ({s}) must start with lowercase");
                                }
                                Some(s.to_string())
                            }
                        }
                        None => None,
                    };
                    // get breaking change indicator
                    breaking_change = match capts.name("breaking") {
                        Some(m) => {
                            let s = m.as_str().trim().to_string();
                            if s.is_empty() {
                                None
                            } else {
                                Some("".to_string())
                            }
                        }
                        None => None,
                    };
                    // eprintln!("type: {:?}", r#type);
                    // eprintln!("scope: {:?}", scope);
                    // eprintln!("desc: {:?}", description);
                } else {
                    bail!("Invalid commit: {line}");
                }

                // process subject
                subject = parts[1].trim().to_string();
                if !subject.starts_with_lowercase() {
                    bail!("Invalid commit: subject must start with lowercase");
                }

                continue;
            }

            // line after subject
            if prev_section == Section::Subject {
                // eprintln!("|> after subject line");
                if !line.is_empty() {
                    bail!("Invalid commit: body must be separated by an empty line");
                } else {
                    prev_section = Section::Body;
                    continue;
                }
            }

            // new breaking change line
            if prev_section == Section::Body && line.starts_with("BREAKING CHANGE:") {
                // eprintln!("|> new breaking change line");
                // next line in body is BREAKING CHANGE
                // NB: strip newline on the previous line
                if let Some(b) = &mut body {
                    if let Some(body_stripped) = b.strip_suffix('\n') {
                        body = Some(body_stripped.to_string());
                    } else {
                        bail!("Invalid commit: BREAKING CHANGE must be preceded from the body by an empty line");
                    }
                }
                if breaking_change.is_some() {
                    bail!("Invalid commit: Several breaking changes");
                } else {
                    breaking_change = Some(
                        line.trim_start_matches("BREAKING CHANGE:")
                            .trim()
                            .to_string(),
                    );
                }
                prev_section = Section::FooterBreakingChange;
                continue;
            }

            // new closed issue line
            if (prev_section == Section::Body
                || prev_section == Section::FooterBreakingChange
                || prev_section == Section::FooterCloseIssue)
                && line.starts_with("Closes #")
            {
                // eprintln!("|> new closed issue line");
                // NB: strip newline on the previous line if the issue is after the body
                if prev_section == Section::Body {
                    if let Some(b) = &mut body {
                        if let Some(body_stripped) = b.strip_suffix('\n') {
                            body = Some(body_stripped.to_string());
                        } else {
                            bail!("Invalid commit: Closes must be preceded from the body by an empty line");
                        }
                    }
                }
                let issue_str = line.trim_start_matches("Closes #");
                let issue_nb = match issue_str.parse::<u32>() {
                    Ok(id) => id,
                    Err(_) => {
                        bail!("Invalid commit: invalid issue number");
                    }
                };
                if let Some(issues) = &mut closed_issues {
                    issues.push(issue_nb);
                } else {
                    closed_issues = Some(vec![issue_nb]);
                }
                // next line in body is ISSUE
                prev_section = Section::FooterCloseIssue;
                continue;
            }

            // breaking change multiline
            if prev_section == Section::FooterBreakingChange {
                // eprintln!("|> breaking change multiline");
                // next line in footer is part of BREAKING CHANGE
                if let Some(b) = &mut breaking_change {
                    b.push('\n');
                    b.push_str(&line);
                } else {
                    unreachable!("breaking change should be set");
                }
                continue;
            }

            if prev_section == Section::Body {
                // eprintln!("|> body line");
                let mut b = if let Some(body_inner) = &body {
                    let mut b = body_inner.clone();
                    b.push('\n');
                    b
                } else {
                    "".to_string()
                };

                b.push_str(&line);
                body = Some(b);
                continue;
            }

            unreachable!("Unreachable line");
        }

        Ok(Self {
            r#type,
            scope,
            subject,
            body,
            breaking_change,
            closed_issues,
        })
    }
}

/// A list of commits
#[derive(Debug)]
pub struct Commits {
    /// Commits
    commits: Vec<Commit>,
}

/// Commit
#[derive(Debug, Clone)]
pub struct Commit {
    /// Hash
    pub hash: String,
    /// Date
    pub date: OffsetDateTime,
    /// Author
    pub author: String,
    /// Raw message
    pub raw_message: String,
    /// Conventional commit message
    pub conv_message: Option<ConvCommitMessage>,
    /// Annotated tag
    pub tag: Option<CommitTag>,
    /// Increments the version
    pub incr_version: VersionIncr,
}

/// Commit tag
#[derive(Debug, Clone)]
pub struct CommitTag {
    /// Tag
    pub tag: String,
    /// Date
    pub date: OffsetDateTime,
    /// Tag message
    pub message: String,
}

/// A list of commits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VersionIncr {
    /// Incompatible API changes
    Major,
    /// Backward compatible additions
    Minor,
    /// Backward compatible fixes
    Patch,
    /// Cannot determine from the commit message
    Unknown,
}

impl Commits {
    /// Loads all commits
    ///
    /// The logs are ordered by most recent first
    pub fn load(config: &CommitsConfig) -> anyhow::Result<Self> {
        let raw_commits = git::log(..)?;
        let tags_refs = git::show_ref_tags()?;
        let tags = git::tag_list()?;

        let mut commits = vec![];
        for c in raw_commits {
            let conv_message = match ConvCommitMessage::parse(&c.message, config) {
                Ok(m) => Some(m),
                Err(_err) => {
                    // eprintln!("[{}] {}", c.hash, err);
                    // eprintln!("{}", c.message);
                    None
                }
            };

            // Assign the commit tag
            let tag = tags_refs
                .iter()
                .find(|(hash, git_tag_ref)| git_tag_ref.is_commit && hash.to_string() == c.hash)
                .map(|(_, git_tag_ref)| &git_tag_ref.tag);

            let tag = if let Some(t) = tag {
                let tag_info = tags.get(t).unwrap();
                Some(CommitTag {
                    tag: tag_info.tag.clone(),
                    date: tag_info.date,
                    message: tag_info.message.clone(),
                })
            } else {
                None
            };

            // Check how a commit increases the next version
            let incr_version = if let Some(conv_msg) = conv_message.as_ref() {
                if conv_msg.breaking_change.is_some() {
                    VersionIncr::Major
                } else if config.increment_minor.contains(&conv_msg.r#type) {
                    VersionIncr::Minor
                } else {
                    VersionIncr::Patch
                }
            } else {
                VersionIncr::Unknown
            };

            commits.push(Commit {
                hash: c.hash,
                date: c.date,
                author: c.author,
                raw_message: c.message,
                conv_message,
                tag,
                incr_version,
            })
        }

        Ok(Commits { commits })
    }

    /// Retrieves the latest release tag
    ///
    /// A release tag is an annotated tag with a semver version, prefixed or not by 'v'.
    ///
    /// The latest tag is the tag with the highest semver version (NOT the most recent)
    pub fn latest_release_tag(&self) -> anyhow::Result<Option<String>> {
        // Get the tag refs
        let refs = git::show_ref_tags()?;

        // Filter out lightweight tags,
        // and collect as a BTreeMap indexed(ordered) by Version
        let versions = refs
            .into_iter()
            .filter_map(|(_hash, tag_ref)| {
                if !tag_ref.is_commit {
                    return None;
                }

                let version_str = if tag_ref.tag.starts_with('v') {
                    tag_ref.tag.strip_prefix('v').unwrap().to_string()
                } else {
                    tag_ref.tag.clone()
                };

                let version = match Version::from_str(&version_str) {
                    Ok(v) => v,
                    Err(_) => {
                        // NB: the git tag is not a semver version => skipped
                        return None;
                    }
                };

                Some((version, tag_ref.tag))
            })
            .collect::<BTreeMap<Version, String>>();

        if versions.is_empty() {
            Ok(None)
        } else {
            let (_, tag) = versions.iter().next_back().unwrap();
            Ok(Some(tag.clone()))
        }
    }

    /// Retrieves the next release tag
    pub fn next_version(&self) -> anyhow::Result<Version> {
        let latest_tag = match self.latest_release_tag()? {
            Some(t) => t,
            None => {
                return Ok(Version::new(0, 1, 0));
            }
        };

        // Get a list of unreleased commits
        let mut unreleased_commits: Vec<&Commit> = vec![];
        'loop_commits: for c in &self.commits {
            if let Some(t) = &c.tag {
                // NB: commit has a tag
                if t.tag == latest_tag {
                    // NB: this commit is the first commit in the latest release
                    break 'loop_commits;
                } else {
                    unreleased_commits.push(c);
                }
            } else {
                // Commit has no tag
                unreleased_commits.push(c);
            }
        }

        // Find out if the unreleased commits should increment the minor or major
        let mut has_major_change = false;
        let mut has_minor_change = false;
        for c in unreleased_commits {
            match c.incr_version {
                VersionIncr::Major => has_major_change = true,
                VersionIncr::Minor => has_minor_change = true,
                VersionIncr::Patch => {}
                VersionIncr::Unknown => {
                    // Commit message is not a conventional message => excluded from versioning
                    eprintln!(
                        "{} Invalid conventional commit [{}]: {}",
                        "⚠".bright_yellow(),
                        c.hash,
                        c.raw_message
                    );
                }
            }
        }

        // Get the current version
        let latest_tag_stripped = if latest_tag.starts_with('v') {
            latest_tag.strip_prefix('v').unwrap().to_string()
        } else {
            latest_tag
        };
        let curr_version = Version::from_str(&latest_tag_stripped)?;

        // Determines the next version
        let mut next_version = curr_version.clone();
        next_version.pre = Prerelease::EMPTY;
        next_version.build = BuildMetadata::EMPTY;
        if curr_version.major > 0 {
            if has_major_change {
                next_version.major += 1;
                next_version.minor = 0;
                next_version.patch = 0;
            } else if has_minor_change {
                next_version.minor += 1;
                next_version.patch = 0;
            } else {
                next_version.patch += 1;
            }
        } else {
            // pre 1.0.0
            // NB: Pre 1.0.0, everything can change at any time
            // We enfore a simple rule that a only a breaking change increments
            // the major
            if has_major_change {
                next_version.minor += 1;
                next_version.patch = 0;
            } else {
                next_version.patch += 1;
            }
        }

        Ok(next_version)
    }
}

impl AsRef<Vec<Commit>> for Commits {
    fn as_ref(&self) -> &Vec<Commit> {
        &self.commits
    }
}
