//! Conventional commits

use std::{collections::BTreeMap, io::BufRead};

use anyhow::bail;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::util::StringExt;

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
#[derive(Debug, PartialEq, Default)]
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

impl ToString for ConvCommitMessage {
    fn to_string(&self) -> String {
        let mut s = String::new();

        // prefix
        s.push_str(
            format!(
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
            )
            .as_str(),
        );

        // body
        if let Some(b) = &self.body {
            s.push_str("\n\n");
            s.push_str(b.as_str());
        }

        // breaking change
        let mut has_breaking_change = false;
        if let Some(b) = &self.breaking_change {
            has_breaking_change = true;
            s.push_str("\n\n");
            s.push_str(format!("BREAKING CHANGE: {}", b).as_str());
        }

        // closed issues
        if let Some(issues) = &self.closed_issues {
            if !issues.is_empty() {
                if !has_breaking_change {
                    s.push('\n');
                }
                for issue in issues {
                    s.push('\n');
                    s.push_str(format!("Closes #{issue}").as_str());
                }
            }
        }

        s
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

        // valid commit keys
        let valid_keys = config
            .types
            .iter()
            .map(|(k, v)| k.clone())
            .collect::<Vec<_>>();
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
                            if !valid_keys.contains(&t.to_string()) {
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
