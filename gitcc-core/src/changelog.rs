//! Changelog

use std::path::Path;

use gitcc_changelog::{Changelog, Release, Section};
use gitcc_git::{discover_repo, get_origin_url};
use indexmap::{indexmap, IndexMap};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{Commit, CommitHistory, Config, Error};

/// Changelog configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct ChangelogConfig {
    /// Release sections
    ///
    /// The key is the group label, and the value is a list of commit types
    ///
    /// # Notes
    ///
    /// [IndexMap] is used to maintain an order of groups
    pub sections: IndexMap<String, Vec<String>>,
}

impl Default for ChangelogConfig {
    fn default() -> Self {
        let sections = indexmap! {
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
        Self { sections }
    }
}

impl ChangelogConfig {
    /// Returns the section label for a specific commit type
    fn find_section_for_commit_type(&self, r#type: &str) -> Option<String> {
        for (section_label, commit_types) in &self.sections {
            if commit_types.contains(&r#type.to_string()) {
                return Some(section_label.clone());
            }
        }
        None
    }
}

/// Changelog build options
#[derive(Debug, Clone, Default)]
pub struct ChangelogBuildOptions {
    /// Origin name (`origin` by default)
    pub origin_name: Option<String>,
    /// Includes all commits
    pub all: bool,
}

/// Builds the changelog
///
/// # Arguments
/// - the `origin` is the origin name (`origin` by default)
pub fn build_changelog(
    cwd: &Path,
    cfg: &Config,
    history: &CommitHistory,
    opts: Option<ChangelogBuildOptions>,
) -> Result<Changelog, Error> {
    let opts = opts.unwrap_or_default();
    let repo = discover_repo(cwd)?;

    let origin_name = opts.origin_name.unwrap_or("origin".to_owned());
    let origin_url = get_origin_url(&repo, &origin_name)?.ok_or(Error::msg(
        format!("remote origin '{origin_name}' not found").as_str(),
    ))?;

    let mut releases = vec![];
    for (release_tag, release_commits) in history
        .commits
        .iter()
        .group_by(|c| c.version_tag.clone())
        .into_iter()
    {
        // eprintln!(
        //     "RELEASE: {}",
        //     release_tag
        //         .as_ref()
        //         .map(|t| t.name.to_string())
        //         .unwrap_or("unreleased".to_string())
        // );

        let mut sections: IndexMap<String, Section> = IndexMap::new();
        for (s_label, _) in &cfg.changelog.sections {
            sections.insert(s_label.to_string(), Section::new(s_label));
        }
        const UNCATEGORIZED: &str = "Uncategorized"; // commits with no type
        const HIDDEN: &str = "__Hidden__"; // ignored commits (types not included)
        sections.insert(UNCATEGORIZED.to_string(), Section::new(UNCATEGORIZED));
        sections.insert(HIDDEN.to_string(), Section::new(HIDDEN));

        for c in release_commits {
            // eprintln!("{}", c.subject());
            let c_sect_label = match &c.conv_message {
                Some(m) => {
                    // eprint!("=> is conventional: {}", m.r#type);
                    match cfg.changelog.find_section_for_commit_type(&m.r#type) {
                        Some(label) => {
                            // eprintln!("=> section: {}", label);
                            label
                        }
                        None => {
                            // eprintln!("=> HIDDEN");
                            HIDDEN.to_string()
                        }
                    }
                }
                None => UNCATEGORIZED.to_string(),
            };

            if c_sect_label == HIDDEN && !opts.all {
                continue;
            }

            let section = sections.get_mut(&c_sect_label).unwrap();
            section.items.push(commit_oneliner(&origin_url, c));
        }

        // remove empty sections
        let sections: Vec<_> = sections
            .into_iter()
            .filter_map(|(_, v)| if v.items.is_empty() { None } else { Some(v) })
            .collect();

        let release_version = release_tag
            .as_ref()
            .map(|t| t.name.to_string())
            .unwrap_or("Unreleased".to_string());
        let release_date = release_tag
            .as_ref()
            .map(|t| t.date)
            .unwrap_or(OffsetDateTime::now_utc());
        let release_url = release_tag
            .as_ref()
            .map(|t| build_release_url(&origin_url, &t.name));

        let release = Release {
            version: release_version,
            date: release_date,
            url: release_url,
            sections,
        };
        releases.push(release);
    }

    Ok(Changelog { releases })
}

/// Builds the release URL
///
/// eg. https://github.com/nlargueze/gitcc/releases/tag/v0.0.15
/// eg. https://github.com/nlargueze/repo/compare/v0.1.1...v0.1.2
fn build_release_url(origin_url: &str, version: &str) -> String {
    format!("{origin_url}/releases/tag/{version}")
}

/// Builders the commit
///
/// eg: chore!: refactoring [#e88da](https://github.com/nlargueze/repo/commit/e88dae6d48fd85b094f58eab029a883969436101)
fn commit_oneliner(origin_url: &str, commit: &Commit) -> String {
    format!(
        "{} [{}]({}/commit/{})",
        commit.subject(),
        commit.short_id(),
        origin_url,
        commit.id
    )
}

#[cfg(test)]
mod tests {
    use crate::commit_history;

    use super::*;

    #[test]
    fn test_changelog() {
        let cwd = std::env::current_dir().unwrap();
        let cfg = Config::load_from_fs(&cwd).unwrap().unwrap_or_default();
        let history = commit_history(&cwd, &cfg).unwrap();
        let _changelog = build_changelog(&cwd, &cfg, &history, None).unwrap();
        // eprintln!("{:#?}", changelog);
        // let changelog_str = changelog.generate(TEMPLATE_CHANGELOG_STD).unwrap();
        // eprintln!("{}", changelog_str);
    }
}
