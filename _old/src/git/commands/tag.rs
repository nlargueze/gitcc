//! Wrappers for `git tag` commands

use std::{collections::HashMap, process::Command};

use chrono::{DateTime, FixedOffset, Utc};

use crate::{
    error::{Error, Result},
    git::tag::GitTag,
};

/// Wrapper for `git tag --list`
pub fn git_get_tags() -> Result<Vec<GitTag>> {
    // get all tags
    let output_git_tag = Command::new("git")
        .args([
            "tag",
            "--list",
            "--format=%(refname:short)|%(creatordate:iso-strict)|%(objectname)",
        ])
        .output()?;
    if !output_git_tag.status.success() {
        return Err(Error::InternalError("Failed to get git tags".to_string()));
    }

    let output_git_tag_str = String::from_utf8_lossy(&output_git_tag.stdout);
    if output_git_tag_str.lines().count() == 0 {
        return Ok(vec![]);
    }

    // map tag hashes to cmmit hashes
    let output_show_ref = Command::new("git")
        .args(["show-ref", "--tags", "--dereference"])
        .output()?;
    if !output_show_ref.status.success() {
        return Err(Error::InternalError(
            "Failed to get git tag refs".to_string(),
        ));
    }
    let hash_ref_map: HashMap<_, _> = String::from_utf8_lossy(&output_show_ref.stdout)
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.splitn(2, ' ').collect();
            if parts.len() != 2 {
                panic!("Invalid tag line: {}", line);
            }
            let hash = parts[0];
            let r#ref = parts[1];
            (hash.to_string(), r#ref.to_string())
        })
        .collect();

    let mut annotated_tags_commits: HashMap<String, String> = HashMap::new();
    for (hash, r#ref) in hash_ref_map {
        if r#ref.ends_with("^{}") {
            let tag_short_ref = r#ref
                .strip_prefix("refs/tags/")
                .unwrap()
                .strip_suffix("^{}")
                .unwrap()
                .to_string();
            annotated_tags_commits.insert(tag_short_ref, hash);
        }
    }

    let tags: Vec<_> = output_git_tag_str
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.splitn(3, '|').collect();
            if parts.len() != 3 {
                panic!("Invalid tag line: {}", line);
            }
            let tag_str = parts[0];
            let dt_str = parts[1];
            let hash_str = parts[2];

            GitTag {
                tag: tag_str.to_string(),
                hash: hash_str.to_string(),
                commit_hash: annotated_tags_commits
                    .get(tag_str)
                    .cloned()
                    .unwrap_or_else(|| hash_str.to_string()),
                date: DateTime::<FixedOffset>::parse_from_rfc3339(dt_str)
                    .unwrap()
                    .with_timezone(&Utc),
                message: None,
            }
        })
        .collect();

    Ok(tags)
}

/// Wrapper for `git tag $t -a -m $m`
pub fn git_set_tag(tag: &str, msg: &str) -> Result<()> {
    let output = Command::new("git")
        .args(["tag", tag, "-a", "-m", msg])
        .output()?;

    let stderr = String::from_utf8(output.stderr).expect("Invalid stderr");

    if !output.status.success() {
        return Err(Error::InternalError(format!(
            "Failed to set git tag: {stderr}"
        )));
    }

    Ok(())
}
