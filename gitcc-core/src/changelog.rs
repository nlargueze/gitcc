//! Changelog

use std::path::Path;

use gitcc_changelog::{Changelog, Release, ReleaseSection};
use gitcc_git::{discover_repo, repo_origin_url};
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
    let origin_url = repo_origin_url(&repo, &origin_name)?.ok_or(Error::msg(
        format!("remote origin '{origin_name}' not found").as_str(),
    ))?;

    let mut releases = vec![];
    for (key_version, group_release) in history
        .commits
        .iter()
        .group_by(|c| c.version.clone().map(|v| v.to_string()))
        .into_iter()
    {
        eprintln!(
            "RELEASE {}",
            key_version.clone().unwrap_or("unreleased".to_owned())
        );

        let mut sections = vec![];

        for (section_label, commits) in group_release
            .group_by(|c| match &c.conv_message {
                Some(msg) => cfg
                    .changelog
                    .find_section_for_commit_type(&msg.r#type)
                    .unwrap_or("___IGNORED___".to_string()),
                None => "Uncategorized".to_string(),
            })
            .into_iter()
        {
            eprintln!("     SECTION: {section_label}");
            let mut section = ReleaseSection {
                label: section_label,
                items: vec![],
            };

            for commit in commits {
                let commit_item = commit_oneliner(&origin_url, commit);
                section.items.push(commit_item);
            }

            sections.push(section);
        }

        let release_version = key_version.unwrap_or("unreleased".to_string());
        let release_date = OffsetDateTime::now_utc();
        let release_url = build_release_url(
            &origin_url,
            history
                .curr_version
                .as_ref()
                .map(|v| format!("v{v}"))
                .unwrap_or("".to_string())
                .as_str(),
            "HEAD",
        );
        let release = Release {
            version: release_version,
            date: release_date,
            url: Some(release_url),
            sections,
        };
        releases.push(release);
    }

    Ok(Changelog { releases })
}

/// Builds the release URL
///
/// eg. https://github.com/nlargueze/repo/compare/v0.1.1...v0.1.2
fn build_release_url(origin_url: &str, from: &str, to: &str) -> String {
    format!("{origin_url}/compare/{from}...{to}")
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
