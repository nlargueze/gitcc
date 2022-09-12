//! Git commands

use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Bound, RangeBounds},
    path::Path,
    process::Command,
};

use anyhow::bail;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

/// Wrapper for `git add -A`
///
/// Returns stdout as the result
pub fn add() -> anyhow::Result<String> {
    let output = Command::new("git")
        .args(["add", "-A", "--verbose"])
        .output()?;

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    if !output.status.success() {
        bail!(format!("{stdout}{stderr}"));
    }

    Ok(stdout)
}

/// Wrapper for `git commit`
///
/// Returns stdout as the result
pub fn commit(msg: &str) -> anyhow::Result<String> {
    let mut cmd = Command::new("git");
    cmd.args(["commit", "-m", msg]);
    let output = cmd.output()?;

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    if !output.status.success() {
        bail!(format!("{stdout}{stderr}"));
    }

    Ok(stdout)
}

/// Wrapper for `git push`
pub fn push(follow_tags: bool) -> anyhow::Result<String> {
    let mut args = vec!["push"];
    if follow_tags {
        args.push("--follow-tags")
    }
    let output = Command::new("git").args(args).output()?;

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    if !output.status.success() {
        bail!(format!("{stdout}{stderr}"));
    }

    Ok(stdout)
}

/// Wrapper for `git config core.hookspath ${dir}`
pub fn config_hooks_path(dir: &Path) -> anyhow::Result<()> {
    let dir_str = dir.to_string_lossy();
    let dir_str = dir_str.as_ref();
    let output = Command::new("git")
        .args(["config", "core.hookspath", dir_str])
        .output()?;

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    if !output.status.success() {
        bail!(format!("{stdout}{stderr}"));
    }

    Ok(())
}

/// Wrapper for `git status --porcelain`
///
/// Returns a list of files that are pending to be committed.
pub fn status_porcelain() -> anyhow::Result<Option<String>> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()?;

    let stdout = String::from_utf8(output.stdout).expect("Invalid stdout");
    let stderr = String::from_utf8(output.stderr).expect("Invalid stderr");

    if !output.status.success() {
        bail!(format!("{stdout}{stderr}"));
    }

    if stdout.is_empty() {
        Ok(None)
    } else {
        Ok(Some(stdout))
    }
}

/// Git tag
///
/// A ligtweight tag inherits its hash, date, and subject from the commit
#[derive(Debug, Clone, Eq)]
pub struct GitTag {
    /// Tag
    pub tag: String,
    /// Hash
    pub hash: String,
    /// Date
    pub date: OffsetDateTime,
    /// Message
    pub message: String,
}

impl PartialEq for GitTag {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Display for GitTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.tag, self.message)
    }
}

/// Wrapper for `git tag --list`
///
/// Returns a map of tag => tag info
pub fn tag_list() -> anyhow::Result<HashMap<String, GitTag>> {
    let output = Command::new("git")
        .args([
            "tag",
            "--list",
            // eg: v1|####|2022-01-01|subject
            // => tag + tag hash + tag date + subject
            "--format=%(refname:short)|%(objectname)|%(creatordate:iso-strict)|%(subject)",
        ])
        .output()?;
    if !output.status.success() {
        bail!("Failed to get git tags");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut tags = HashMap::new();
    for line in stdout.lines() {
        // Each line is formatted as `tag|date|hash`
        let parts: Vec<_> = line.splitn(4, '|').collect();

        if parts.len() != 4 {
            panic!("Invalid tag line: {}", line);
        }
        let tag = parts[0];
        let hash = parts[1];
        let date = OffsetDateTime::parse(parts[2], &Rfc3339)?;
        let message = parts[3];

        tags.insert(
            tag.to_string(),
            GitTag {
                tag: tag.to_string(),
                hash: hash.to_string(),
                date,
                message: message.to_string(),
            },
        );
    }

    Ok(tags)
}

/// Git tag reference
#[derive(Debug, Clone)]
pub struct GitTagRef {
    /// Tag (short)
    pub tag: String,
    /// Is commit hash
    pub is_commit: bool,
}

/// Wrapper for `git show-ref --tags --dereference`
///
/// Returns a map (commit or tag hash => [GitTagRef])
pub fn show_ref_tags() -> anyhow::Result<HashMap<String, GitTagRef>> {
    let output = Command::new("git")
        .args(["show-ref", "--tags", "--dereference"])
        .output()?;
    if !output.status.success() {
        bail!("Failed to get git tags refs");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Map (tag hash => GitTagRef)
    let mut refs = HashMap::new();

    for line in stdout.lines() {
        let parts: Vec<_> = line.splitn(2, ' ').collect();
        if parts.len() != 2 {
            panic!("Invalid tag line: {}", line);
        }
        let hash = parts[0];
        let r#ref = parts[1];

        let (tag, is_commit) = if r#ref.ends_with("^{}") {
            // if the ref ends with '^{}', the hash is the commit hash
            let tag_short = r#ref
                .strip_suffix("^{}")
                .unwrap()
                .strip_prefix("refs/tags/")
                .unwrap()
                .to_string();
            (tag_short, true)
        } else {
            // This is a normal tag (lightweight or annotated)
            let tag_short = r#ref.strip_prefix("refs/tags/").unwrap().to_string();
            (tag_short, false)
        };

        refs.insert(hash.to_string(), GitTagRef { tag, is_commit });
    }

    Ok(refs)
}

/// Wrapper for `git tag $t -a -m $m`
///
/// Sets an annotated tag
pub fn tag(tag: &str, msg: &str) -> anyhow::Result<String> {
    let output = Command::new("git")
        .args(["tag", tag, "-a", "-m", msg])
        .output()?;

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    if !output.status.success() {
        bail!(format!("{stdout}{stderr}"));
    }

    Ok(stdout)
}

/// A git commit
#[derive(Debug, Eq)]
pub struct GitCommit {
    /// Hash
    pub hash: String,
    /// Date
    pub date: OffsetDateTime,
    /// Author
    pub author: String,
    /// Message
    pub message: String,
}

impl PartialEq for GitCommit {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Display for GitCommit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.hash)?;
        writeln!(
            f,
            "{}",
            self.date
                .format(&Rfc3339)
                .map_err(|_err| { std::fmt::Error })?
        )?;
        writeln!(f, "{}", self.author)?;
        write!(f, "{}", self.message)?;
        Ok(())
    }
}

/// Wrapper for `git log`
///
/// # Commit range
///
/// - `git log id1..`: get all logs from ref `id1` (exclusive) to HEAD
/// - `git log id1..id2`: get all logs from ref `id1` (exclusive) to the ref `id2` (inclusive)
///
/// # Order
///
/// The [Vec] is ordered with the most recent commit first
pub fn log<R: RangeBounds<String>>(range: R) -> anyhow::Result<Vec<GitCommit>> {
    // Parse the range
    let log_range = match (range.start_bound(), range.end_bound()) {
        (Bound::Included(s), Bound::Included(e)) => format!("{s}..{e}"),
        (Bound::Included(s), Bound::Unbounded) => format!("{s}.."),
        (Bound::Unbounded, Bound::Included(e)) => format!("..{e}"),
        (Bound::Unbounded, Bound::Unbounded) => "".to_string(),
        _ => {
            bail!("Invalid log range")
        }
    };

    /// Commits separator
    const SEPARATOR: &str = "~~~~~~~~~~~~~~~~";

    let mut cmd = Command::new("git");
    cmd.args([
        "log",
        // The format is
        // - hash
        // - date
        // - author
        // - message
        // [SEPARATOR]
        format!("--format=%H%n%ad%n%an%n%B{SEPARATOR}").as_str(),
        "--date=iso-strict",
    ]);
    if !log_range.is_empty() {
        cmd.arg(log_range);
    }
    let output = cmd.output()?;

    if !output.status.success() {
        bail!("Failed to get git logs");
    }

    let mut commits: Vec<GitCommit> = Vec::new();
    let mut commit = GitCommit {
        hash: String::new(),
        date: OffsetDateTime::now_utc(),
        author: String::new(),
        message: String::new(),
    };
    enum CommitLine {
        Hash,
        Date,
        Author,
        // the index is the line number
        Body(u32),
    }
    let mut commit_line = CommitLine::Hash;

    for line in String::from_utf8(output.stdout)?.lines() {
        // eprintln!("|> {}", line);

        if line == SEPARATOR {
            commits.push(commit);
            commit = GitCommit {
                hash: String::new(),
                date: OffsetDateTime::now_utc(),
                author: String::new(),
                message: String::new(),
            };
            commit_line = CommitLine::Hash;
            continue;
        }

        match commit_line {
            CommitLine::Hash => {
                commit.hash = line.to_string();
                commit_line = CommitLine::Date;
            }
            CommitLine::Date => {
                commit.date = OffsetDateTime::parse(line, &Rfc3339)?;
                commit_line = CommitLine::Author;
            }
            CommitLine::Author => {
                commit.author = line.to_string();
                commit_line = CommitLine::Body(0);
            }
            CommitLine::Body(i) => {
                if i > 0 {
                    commit.message.push('\n');
                }
                commit.message.push_str(line);
                commit_line = CommitLine::Body(i + 1);
            }
        }
    }

    Ok(commits)
}

/// Returns the repo origin URL
pub fn config_origin_url() -> anyhow::Result<String> {
    let output = Command::new("git")
        .args(["config", "--get", "remote.origin.url"])
        .output()?;
    if !output.status.success() {
        bail!("Failed to get repo origin URL");
    }

    let stdout = String::from_utf8(output.stdout)?;
    let mut origin = stdout.trim();
    if origin.ends_with(".git") {
        origin = origin.strip_suffix(".git").unwrap()
    }

    Ok(origin.to_string())
}
