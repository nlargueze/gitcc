//! Changelog
//!
//! The changelog format is defined in https://keepachangelog.com/en/1.0.0/.

use std::str::FromStr;

use anyhow::bail;
use colored::Colorize;
use handlebars::Handlebars;
use indexmap::{indexmap, IndexMap};
use indoc::indoc;
use semver::Version;
use serde::{Deserialize, Serialize, Serializer};
use time::{macros::format_description, OffsetDateTime};

use crate::{commit::Commits, git};

/// Changelog configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct ChangelogConfig {
    /// Commit groups
    ///
    /// The key is the group label,
    /// and the value is a list of commit types
    pub groups: IndexMap<String, Vec<String>>,
}

impl Default for ChangelogConfig {
    fn default() -> Self {
        let groups = indexmap! {
            "New features".to_string() => vec!["feat".to_string()],
            "Bug fixes".to_string() => vec!["fix".to_string()],
            "Documentation".to_string() => vec!["docs".to_string()],
            "Performance improvements".to_string() => vec!["perf".to_string()],
            "Tooling".to_string() => vec![
                "build".to_string(),
                "ci".to_string(),
                "cd".to_string()
            ],
        };
        Self { groups }
    }
}

/// Changelog
#[derive(Debug, Serialize)]
pub struct Changelog {
    /// Releases (latest first)
    releases: Vec<Release>,
}

/// Changelog release
#[derive(Debug, Serialize)]
struct Release {
    /// Release ref
    #[serde(serialize_with = "serialize_ref")]
    r#ref: String,
    /// Version
    version: Option<Version>,
    /// Date
    #[serde(serialize_with = "serialize_date")]
    date: OffsetDateTime,
    /// The url to see the release changes
    ///
    /// [Unreleased]: https://github.com/olivierlacan/keep-a-changelog/compare/v1.0.0...HEAD
    /// [1.0.0]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.0.2...v0.0.1
    url: String,
    /// Groups
    groups: Vec<ReleaseGroup>,
}

/// Removes the prefix 'v' of a tag
fn serialize_ref<S>(tag: &String, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if tag == "HEAD" {
        "Unreleased".serialize(serializer)
    } else {
        tag.strip_prefix('v').unwrap_or(tag).serialize(serializer)
    }
}

/// Serializes a date
fn serialize_date<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let format = format_description!("[year]-[month]-[day]");
    date.format(&format).unwrap().serialize(serializer)
}

/// Changelog release group
#[derive(Debug, Serialize)]
struct ReleaseGroup {
    /// Group label
    label: String,
    /// Group commits
    commits: Vec<ReleaseCommit>,
}

/// Changelog commit
#[derive(Debug, Serialize)]
struct ReleaseCommit {
    /// Hash
    hash: String,
    /// Hash (short)
    hash_short: String,
    /// Type
    r#type: String,
    /// Scope
    scope: Option<String>,
    /// Subject
    subject: String,
    /// Commit url (.../commit/#hash)
    url: String,
}

impl Changelog {
    /// Instantiates a new [Changelog] from commits
    pub fn new(
        config: &ChangelogConfig,
        commits: &Commits,
        tag_next: bool,
    ) -> anyhow::Result<Self> {
        let next_version = if tag_next {
            Some(commits.next_version()?)
        } else {
            None
        };

        let next_tag = next_version
            .clone()
            .map(|v| format!("v{}", v))
            .unwrap_or_else(|| "HEAD".to_string());

        // Get the repo origin
        let origin_url = git::config_origin_url()?;

        // Parse all commits
        let mut releases = vec![];
        let mut this_release = Release {
            r#ref: next_tag,
            version: next_version,
            date: OffsetDateTime::now_utc(),
            url: "__".to_string(),
            groups: vec![],
        };
        for c in commits.as_ref() {
            // Check if the commit is tagged => new release
            if let Some(tag) = &c.tag {
                // NB: Commit has an annotated tag => this commit belongs to a new release
                this_release.url = format!(
                    "{}/compare/{}...{}",
                    origin_url, tag.tag, this_release.r#ref
                );

                // Sort the groups according to the provided config
                this_release.groups.sort_by_key(|g| {
                    if let Some(i) = config.groups.get_index_of(&g.label) {
                        return i;
                    }
                    config.groups.len()
                });

                // Add release to list & init new release
                releases.push(this_release);
                this_release = Release {
                    r#ref: tag.tag.clone(),
                    version: Version::from_str(tag.tag.strip_prefix('v').unwrap_or(&tag.tag)).ok(),
                    date: tag.date,
                    url: "".to_string(),
                    groups: vec![],
                }
            }

            // Get commit info
            let hash = c.hash.clone();
            let mut hash_short = c.hash.clone();
            hash_short.truncate(5);

            let (r#type, subject, scope) = if let Some(conv_msg) = &c.conv_message {
                (
                    conv_msg.r#type.clone(),
                    conv_msg.subject.clone(),
                    conv_msg.scope.clone(),
                )
            } else {
                eprintln!(
                    "{} Invalid conventional commit [{}]: {}",
                    "⚠".bright_yellow(),
                    hash_short,
                    c.raw_message
                );
                let subject = c.raw_message.lines().next().unwrap().to_string();
                ("uncategorized".to_string(), subject, None)
            };

            let url = format!("{}/commit/{}", origin_url, c.hash);

            let commit = ReleaseCommit {
                hash,
                hash_short,
                r#type: r#type.clone(),
                scope,
                subject,
                url,
            };

            // Find and assign to commit group
            let group_label = config
                .groups
                .iter()
                .find_map(|(cfg_group_label, cfg_group_keys)| {
                    if cfg_group_keys.contains(&r#type) {
                        Some(cfg_group_label.as_str())
                    } else if r#type == "uncategorized" {
                        Some("Uncategorized")
                    } else {
                        None
                    }
                });
            if let Some(group_label) = group_label {
                // Commit has a group and is added to the changelog
                if let Some(group) = this_release
                    .groups
                    .iter_mut()
                    .find(|g| g.label.as_str() == group_label)
                {
                    group.commits.push(commit);
                } else {
                    this_release.groups.push(ReleaseGroup {
                        label: group_label.to_string(),
                        commits: vec![commit],
                    })
                }
            }
        }

        Ok(Self { releases })
    }

    /// Retrieves the latest version
    pub fn latest_version(&self) -> Option<Version> {
        self.releases.first().and_then(|r| r.version.clone())
    }
}

// --------------------------------------------------
// GENERATION
// --------------------------------------------------

/// Changelog template
const CHANGELOG_TEMPLATE: &str = indoc!(
    "# Changelog
    
    All notable changes to this project will be documented in this file.

    {{#each releases}}
    ## [{{this.ref}}] - {{this.date}}
    {{#if this.url}}

    {{this.url}}
    {{/if}}

    {{#each this.groups}}
    ### {{this.label}}

    {{#each this.commits}}
    - {{this.scope}}{{this.subject}} [#{{this.hash_short}}]({{this.url}})
    {{/each}}

    {{/each}}
    {{/each}}"
);

/// Latest release template
const LATEST_RELEASE_TEMPLATE: &str = indoc!(
    "# Release notes - {{this.ref}}
    
    {{this.date}}

    {{this.url}}

    {{#each this.groups}}
    ## {{this.label}}

    {{#each this.commits}}
    - {{this.scope}}{{this.subject}} [#{{this.hash_short}}]({{this.url}})
    {{/each}}
    {{/each}}"
);

/// Change log format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangelogFormat {
    /// Full changelog
    Full,
    /// Release note
    LatestRelease,
}

impl Changelog {
    /// Generates the change log
    pub fn generate(&self, format: ChangelogFormat) -> anyhow::Result<String> {
        let handlebars = Handlebars::new();
        let file = match format {
            ChangelogFormat::Full => {
                //
                handlebars.render_template(CHANGELOG_TEMPLATE, &self)?
            }
            ChangelogFormat::LatestRelease => {
                let release = self.releases.first();
                if let Some(release) = release {
                    handlebars.render_template(LATEST_RELEASE_TEMPLATE, &release)?
                } else {
                    bail!("No release");
                }
            }
        };

        Ok(file)
    }
}
