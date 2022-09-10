//! Changelog generation
//!
//! ## Format
//!
//! The format is inspired by:
//!
//! Refer to [https://keepachangelog.com/en/1.0.0/](https://keepachangelog.com/en/1.0.0/)

use chrono::Utc;
use handlebars::Handlebars;
use indoc::indoc;
use log::warn;
use serde::Serialize;

use crate::{
    config::Config,
    conventional::ConventionalCommitMessage,
    error::Result,
    git::{get_config_origin_url, git_get_tags, git_log},
    utils::StringExt,
    version::IntoSemverGitTags,
};

/// Changelog template
const CHANGELOG_TEMPLATE: &str = indoc!(
    "# Changelog
    
    All notable changes to this project will be documented in this file.

    {{#each releases}}
    ## [{{this.version}}] - {{this.date}}
    {{#if this.history_url}}

    {{this.history_url}}
    {{/if}}

    {{#each this.groups}}
    ### {{this.title}}

    {{#each this.commits}}
    - {{this.prefix}}{{this.subject}} {{this.commit_link}}
    {{/each}}

    {{/each}}
    {{/each}}"
);

/// Release Notes template
const RELEASENOTES_TEMPLATE: &str = indoc!(
    "Release notes for `v{{this.version}}`
    
    {{#each this.groups}}
    ### {{this.title}}

    {{#each this.commits}}
    - {{this.prefix}}{{this.subject}} {{this.commit_link}}
    {{/each}}

    {{/each}}
    "
);

/// Changelog commit
#[derive(Debug, Clone, Serialize)]
struct ChangeLogCommit {
    r#type: String,
    prefix: String,
    subject: String,
    commit_link: String,
}

/// Changelog release group
#[derive(Debug, Clone, Serialize)]
struct ChangeLogReleaseGroup {
    key: String,
    title: String,
    commits: Vec<ChangeLogCommit>,
}

/// Changelog release
#[derive(Debug, Clone, Serialize)]
struct ChangeLogRelease {
    /// Release version
    version: String,
    /// Release date
    date: String,
    /// Release commit history link
    history_url: String,
    /// Commits group
    groups: Vec<ChangeLogReleaseGroup>,
}

/// ChangeLog data
#[derive(Debug, Serialize)]
struct ChangeLogData {
    /// Commits grouped by release
    releases: Vec<ChangeLogRelease>,
}

/// Release notes data
#[derive(Debug, Serialize)]
struct ReleaseNotesData {
    /// Release version
    version: String,
    /// Commits groups
    groups: Vec<ChangeLogReleaseGroup>,
}

/// A change log template.
#[derive(Debug)]
pub struct ChangeLog {
    /// Change log template engine.
    registry: Handlebars<'static>,
}

impl ChangeLog {
    /// Initializes the changelog.
    pub fn init() -> Result<Self> {
        // init template registry
        let mut registry = Handlebars::new();
        // registry.set_strict_mode(true);
        registry.register_template_string("changelog", CHANGELOG_TEMPLATE)?;
        registry.register_template_string("releasenotes", RELEASENOTES_TEMPLATE)?;

        Ok(Self { registry })
    }

    /// Generates the change log file.
    pub fn generate(&self, config: &Config, next_version: &str) -> Result<(String, String)> {
        // parse commits
        let mut data = ChangeLogData { releases: vec![] };
        data.releases.push(ChangeLogRelease {
            // NB: Could use "Unreleased" instead of the next version
            version: next_version.to_string(),
            date: Utc::now().format("%Y-%m-%d").to_string(),
            history_url: "".to_string(),
            groups: vec![],
        });

        // read all logs from the start of the repository (latest to earliest)
        let commits = git_log("")?;

        // read all tags from the repository
        let tags = git_get_tags()?.into_semver()?;

        // Origin URL
        let origin_url = get_config_origin_url()?;

        for c in commits {
            // eprintln!("{:#?}", c);

            // > get type and subject from the message
            let changelog_commit =
                match ConventionalCommitMessage::parse(&c.message, &config.valid_commit_types()) {
                    Ok(conv_msg) => {
                        let mut short_hash = c.id.clone();
                        short_hash.truncate(5);
                        let commit_url = format!("{}/commit/{}", origin_url, c.id);
                        let commit_link = format!("[#{}]({})", short_hash, commit_url);

                        ChangeLogCommit {
                            r#type: conv_msg.r#type.clone(),
                            prefix: "".to_string(),
                            // prefix: conv_msg
                            //     .scope
                            //     .map(|s| format!("{}: ", s))
                            //     .unwrap_or_default(),
                            subject: conv_msg.subject.clone().to_uppercase_first(),
                            commit_link,
                        }
                    }
                    Err(err) => {
                        // NB: add as a specific group
                        let mut short_id = c.id.clone();
                        short_id.truncate(7);
                        warn!("Commit ({}) is unconventional ({})", short_id, err);
                        let commit_msg_first_line = c.message.lines().next().unwrap();
                        ChangeLogCommit {
                            r#type: "uncategorized".to_string(),
                            prefix: "".to_string(),
                            subject: commit_msg_first_line.to_string(),
                            commit_link: "".to_string(),
                        }
                    }
                };

            let type_title = if config.changelog.types.contains(&changelog_commit.r#type) {
                match config.commit.types.get(&changelog_commit.r#type) {
                    Some(x) => x.clone(),
                    None => changelog_commit.r#type.clone(),
                }
            } else {
                // NB: type is excluded from the changelog
                continue;
            };

            let commit_tag = tags.iter().find(|t| t.tag.commit_hash == c.id);
            match commit_tag {
                Some(t) => {
                    // commit has a tag which means that it belongs to another version
                    data.releases.push(ChangeLogRelease {
                        version: t.version.to_string(),
                        date: t.tag.date.format("%Y-%m-%d").to_string(),
                        history_url: "".to_string(),
                        groups: vec![],
                    });
                }
                None => {}
            }

            // add release for that commit
            let release = data.releases.last_mut().unwrap();

            if let Some(group) = release
                .groups
                .iter_mut()
                .find(|g| g.key == changelog_commit.r#type)
            {
                group.commits.push(changelog_commit);
            } else {
                // New group
                let group_title = if changelog_commit.r#type == "uncategorized" {
                    "Uncategorized".to_string()
                } else {
                    type_title
                };

                let group = ChangeLogReleaseGroup {
                    key: changelog_commit.r#type.clone(),
                    title: group_title,
                    commits: vec![changelog_commit],
                };

                release.groups.push(group);
            };
        }

        // debug
        // eprintln!("{:#?}", data);

        // for each release, add history link & sort groups
        // [Unreleased]: https://github.com/olivierlacan/keep-a-changelog/compare/v1.0.0...HEAD
        // [1.0.0]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.0.2...v0.0.1
        let mut from_ref: Option<String> = None;
        for release in data.releases.iter_mut().rev() {
            if let Some(ref_start) = from_ref {
                let ref_end = if release.version == "Unreleased" {
                    "HEAD".to_string()
                } else {
                    release.version.clone()
                };
                release.history_url = format!("{}/compare/{}...{}", origin_url, ref_start, ref_end);
            }
            from_ref = Some(release.version.clone());

            // sort
            release.groups.sort_by(|g1, g2| {
                let i1 = config
                    .changelog
                    .types
                    .iter()
                    .position(|cfg_type| cfg_type == &g1.key)
                    .expect("Invalid type");
                let i2 = config
                    .changelog
                    .types
                    .iter()
                    .position(|cfg_type| cfg_type == &g2.key)
                    .expect("Invalid type");
                i1.cmp(&i2)
            });
        }

        // render changelog
        let changelog = self.registry.render("changelog", &data)?;

        // render release notes
        let this_release = data.releases.first().unwrap();
        let release_notes_date = ReleaseNotesData {
            version: this_release.version.clone(),
            groups: this_release.groups.to_vec(),
        };
        let releasenotes = self.registry.render("releasenotes", &release_notes_date)?;

        Ok((changelog, releasenotes))
    }
}
