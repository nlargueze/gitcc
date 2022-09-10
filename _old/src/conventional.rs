//! Conventional commit parsing and formatting

use std::{io::BufRead, string::ToString};

use regex::Regex;

use crate::{
    error::{Error, Result},
    utils::StringExt,
};

/// Conventional commit message.
#[derive(Debug, PartialEq, Default)]
pub struct ConventionalCommitMessage {
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

impl ToString for ConventionalCommitMessage {
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

impl ConventionalCommitMessage {
    /// Parses a string into a conventional commit message
    pub fn parse(s: &str, valid_types: &[String]) -> Result<Self> {
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
                    return Err(Error::InvalidCommit(
                        "Conventional commit missing ':' separator".to_string(),
                    ));
                }
                // parse the prefix
                let prefix = parts[0].trim().to_string();
                if let Some(capts) = regex_prefix.captures(&prefix) {
                    // get type
                    r#type = match capts.name("type") {
                        Some(m) => {
                            let t = m.as_str();
                            if !valid_types.contains(&t.to_string()) {
                                return Err(Error::InvalidCommit(format!(
                                    "Invalid conventional commit type '{}'",
                                    m.as_str()
                                )));
                            }
                            t.to_string()
                        }
                        None => {
                            return Err(Error::InvalidCommit(format!(
                                "Missing conventional commit type in '{}'",
                                line
                            )));
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
                                    return Err(Error::InvalidCommit(format!(
                                        "Invalid commit: scope ({s}) must start with lowercase"
                                    )));
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
                    return Err(Error::InvalidCommit(format!("Invalid commit: {}", line)));
                }

                // process subject
                subject = parts[1].trim().to_string();
                if !subject.starts_with_lowercase() {
                    return Err(Error::InvalidCommit(
                        "Invalid commit: subject must start with lowercase".to_string(),
                    ));
                }

                continue;
            }

            // line after subject
            if prev_section == Section::Subject {
                // eprintln!("|> after subject line");
                if !line.is_empty() {
                    return Err(Error::InvalidCommit(
                        "Invalid commit: body must be separated by an empty line".to_string(),
                    ));
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
                        return Err(Error::InvalidCommit(
                            "Invalid commit: BREAKING CHANGE must be preceded from the body by an empty line".to_string(),
                        ));
                    }
                }
                if breaking_change.is_some() {
                    return Err(Error::InvalidCommit(
                        "Invalid commit: Several breaking changes".to_string(),
                    ));
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
                            return Err(Error::InvalidCommit(
                                "Invalid commit: Closes must be preceded from the body by an empty line".to_string(),
                            ));
                        }
                    }
                }
                let issue_str = line.trim_start_matches("Closes #");
                let issue_nb = match issue_str.parse::<u32>() {
                    Ok(id) => id,
                    Err(_) => {
                        return Err(Error::InvalidCommit(
                            "Invalid commit: invalid issue number".to_string(),
                        ));
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

        Ok(ConventionalCommitMessage {
            r#type,
            scope,
            subject,
            body,
            breaking_change,
            closed_issues,
        })
    }
}
