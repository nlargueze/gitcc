//! Conventional commits
//!
//! This crate provides the tools to work with conventional commits.

//! Conventional commits
//!
//! This module is based on [Conventional commits](https://www.conventionalcommits.org/en/v1.0.0/)

use std::{collections::BTreeMap, fmt::Display, io::BufRead, str::FromStr};

use indexmap::IndexMap;
use lazy_static::lazy_static;
use regex::Regex;

pub use crate::util::StringExt;

mod util;

lazy_static! {
    /// Default conventional commit types
    pub static ref DEFAULT_CONVCO_TYPES: BTreeMap<&'static str, &'static str> = {
        let mut m = BTreeMap::new();
        m.insert("feat", "New features");
        m.insert("fix", "Bug fixes");
        m.insert("docs", "Documentation");
        m.insert("style", "Code styling");
        m.insert("refactor", "Code refactoring");
        m.insert("perf", "Performance Improvements");
        m.insert("test", "Testing");
        m.insert("build", "Build system");
        m.insert("ci", "Continuous Integration");
        m.insert("cd", "Continuous Delivery");
        m.insert("chore", "Other changes");
        m
    };
}

/// Default commit types which increment the minor version
pub const DEFAULT_CONVCO_INCR_MINOR_TYPES: [&str; 1] = ["feat"];

/// Breaking change key
pub const BREAKING_CHANGE_KEY: &str = "BREAKING CHANGE";

/// Breaking change key (with dash)
pub const BREAKING_CHANGE_KEY_DASH: &str = "BREAKING-CHANGE";

/// Conventional commit message
#[derive(Debug, Default, Clone)]
pub struct ConvcoMessage {
    /// Commit type
    pub r#type: String,
    /// Commit scope
    pub scope: Option<String>,
    /// Indicates that this is a breaking change (!)
    pub is_breaking: bool,
    /// Commit description
    pub desc: String,
    /// Commit body
    pub body: Option<String>,
    /// Footer
    ///
    /// A footer must be a list of key: value pairs following the git trailer convention
    pub footer: Option<IndexMap<String, String>>,
}

impl ConvcoMessage {
    /// Sets a breaking change
    pub fn add_breaking_change(&mut self, desc: &str) -> &mut Self {
        self.is_breaking = true;
        if let Some(entries) = &mut self.footer {
            entries.insert(BREAKING_CHANGE_KEY.to_string(), desc.to_string());
        }
        self
    }

    /// Inserts a footer note
    pub fn add_footer_note(&mut self, key: &str, value: &str) -> &mut Self {
        if let Some(entries) = &mut self.footer {
            entries.insert(key.to_string(), value.to_string());
        } else {
            let mut entries = IndexMap::new();
            entries.insert(key.to_string(), value.to_string());
            self.footer = Some(entries);
        }

        self
    }

    /// Checks if the message has a breaking change
    pub fn is_breaking_change(&self) -> bool {
        if self.is_breaking {
            return true;
        }

        if let Some(entries) = &self.footer {
            return entries.contains_key(BREAKING_CHANGE_KEY);
        }
        false
    }
}

/// Conventional commit error
#[derive(Debug, thiserror::Error)]
#[error("Invalid conventional commit:{0}")]
pub struct ConvcoError(String);

impl Display for ConvcoMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}: {}",
            self.r#type,
            self.scope
                .as_ref()
                .map(|s| format!("({s})"))
                .unwrap_or_default(),
            if self.is_breaking { "!" } else { "" },
            self.desc
        )?;

        // Body
        if let Some(b) = &self.body {
            write!(f, "\n\n")?;
            write!(f, "{b}")?;
        }

        // Footer
        if let Some(entries) = &self.footer {
            write!(f, "\n\n")?;
            let mut it = entries.iter().peekable();
            while let Some((k, v)) = it.next() {
                if it.peek().is_none() {
                    // NB: last entry
                    write!(f, "{k}: {v}")?;
                } else {
                    writeln!(f, "{k}: {v}")?;
                }
            }
        }

        Ok(())
    }
}

// cf. https://2fd.github.io/rust-regex-playground
lazy_static! {
    /// Regex to parse the subject
    ///
    /// The following groups are defined
    /// - type: eg. feat
    /// - scope: eg. (abcd)
    /// - breaking: eg. ! or ""
    /// - subject: eg. long text
    static ref REGEX_SUBJECT: Regex = Regex::new(
        r"(?P<type>[[:word:]]+)(?P<scope>[\(][[:word:]]+[\)])?(?P<breaking>[!])?: (?P<desc>.*)"
    )
    .expect("Invalid regex");
}

lazy_static! {
    static ref REGEX_FOOTER_KV: Regex =
        Regex::new(r"(?P<key>.*): (?P<value>.*)").expect("Invalid regex");
}

impl FromStr for ConvcoMessage {
    type Err = ConvcoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[derive(Debug, PartialEq)]
        enum Section {
            Subject,
            Body,
            Footer,
        }

        let mut r#type = String::new();
        let mut scope: Option<String> = None;
        let mut is_breaking: bool = false;
        let mut desc = String::new();
        let mut body: Option<String> = None;
        let mut footer: Option<IndexMap<String, String>> = None;

        // We parse starting from the bottom
        let mut this_section = Section::Subject;
        let mut is_prev_line_empty = false;
        for (i, line_res) in s.as_bytes().lines().enumerate() {
            let line = line_res.map_err(|e| ConvcoError(e.to_string()))?;

            match (i, &this_section) {
                (0, Section::Subject) => {
                    // => parse 1st line
                    let caps = match REGEX_SUBJECT.captures(s) {
                        Some(caps) => caps,
                        None => {
                            return Err(ConvcoError("invalid subject line".to_string()));
                        }
                    };
                    if let Some(ok) = caps.name("type") {
                        r#type = ok.as_str().to_string();
                        if r#type.is_empty() || !r#type.is_lowercase() {
                            return Err(ConvcoError(
                                "type must non empty and lowercase".to_string(),
                            ));
                        }
                    } else {
                        return Err(ConvcoError("missing subject type".to_string()));
                    };
                    if let Some(ok) = caps.name("scope") {
                        let scope_raw = ok.as_str();
                        let scope_raw = scope_raw.strip_prefix('(').unwrap();
                        let scope_raw = scope_raw.strip_suffix(')').unwrap();
                        if scope_raw.is_empty() || !scope_raw.is_lowercase() {
                            return Err(ConvcoError(
                                "scope must non empty and lowercase".to_string(),
                            ));
                        }
                        scope = Some(scope_raw.to_string());
                    };
                    if caps.name("breaking").is_some() {
                        is_breaking = true;
                    };
                    match caps.name("desc") {
                        Some(ok) => {
                            desc = ok.as_str().to_string();
                            if !desc.starts_with_lowercase() {
                                return Err(ConvcoError(
                                    "subject must start with lowercase".to_string(),
                                ));
                            }
                        }
                        None => {
                            return Err(ConvcoError("missing subject description".to_string()));
                        }
                    };
                }
                (1, Section::Subject) => {
                    // >> line after subject
                    if line.is_empty() {
                        is_prev_line_empty = true;
                        this_section = Section::Body;
                    } else {
                        return Err(ConvcoError(
                            "body must be separated by an empty line".to_string(),
                        ));
                    }
                }
                (_, Section::Subject) => {
                    unreachable!()
                }
                (_, Section::Body) => {
                    // eprintln!("{:#?}", line);

                    // NOTES:
                    // OK here it gets a bit tricky to split the body and the footer.
                    // The footer is the last paragraph and must be a list of key-value pairs (K: V),
                    // where K is a word token (separation is made with '-', with the execption of the value 'BREAKING CHANGE').
                    //
                    // So, we use the heuristics that, for each line, if the previous line is blank,
                    // and if that line starts with a valid key/value pair, we are now part of the footer.
                    if is_prev_line_empty {
                        if let Some(caps) = REGEX_FOOTER_KV.captures(&line) {
                            let key = caps.name("key").unwrap().as_str();
                            if is_valid_footer_token(key) {
                                // => we are part of the footer
                                let value = caps.name("value").unwrap().as_str();
                                let mut m = IndexMap::new();
                                m.insert(key.to_string(), value.to_string());
                                footer = Some(m);
                                is_prev_line_empty = false;
                                this_section = Section::Footer;
                                // NB: removing the '\n' at the end of the body
                                body = body.map(|b| b.trim().to_string());
                                continue;
                            }
                        };
                    }

                    let mut b = if let Some(mut b) = body {
                        b.push('\n');
                        b
                    } else {
                        "".to_string()
                    };
                    b.push_str(&line);
                    body = Some(b);
                    is_prev_line_empty = line.is_empty();
                }
                (_, Section::Footer) => {
                    if let Some(caps) = REGEX_FOOTER_KV.captures(&line) {
                        let key = caps.name("key").unwrap().as_str();
                        if is_valid_footer_token(key) {
                            // => we are part of the footer
                            let value = caps.name("value").unwrap().as_str();
                            if let Some(f) = &mut footer {
                                f.insert(key.to_string(), value.to_string());
                            } else {
                                unreachable!()
                            }
                        } else {
                            return Err(ConvcoError(format!("invalid footer key: '{key}'")));
                        }
                    } else {
                        return Err(ConvcoError("invalid footer line".to_string()));
                    };
                }
            }
        }

        // ⮑
        Ok(Self {
            r#type,
            scope,
            is_breaking,
            desc,
            body,
            footer,
        })
    }
}

/// Checks if a string is valid footer token
///
/// A valid footer token is a word token with no white space, with the exception of the value 'BREAKING CHANGE'
fn is_valid_footer_token(value: &str) -> bool {
    if value.contains(' ') {
        return value == BREAKING_CHANGE_KEY;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex_footer_simple() {
        let s = "Refs: #123";
        let caps = REGEX_FOOTER_KV.captures(s).unwrap();
        // eprintln!("{caps:#?}");
        assert_eq!(caps.name("key").unwrap().as_str(), "Refs");
        assert_eq!(caps.name("value").unwrap().as_str(), "#123");
    }

    #[test]
    fn regex_footer_breaking_change() {
        let s = "BREAKING CHANGE: This is a breaking change";
        let caps = REGEX_FOOTER_KV.captures(s).unwrap();
        // eprintln!("{caps:#?}");
        assert_eq!(caps.name("key").unwrap().as_str(), "BREAKING CHANGE");
        assert_eq!(
            caps.name("value").unwrap().as_str(),
            "This is a breaking change"
        );
    }

    #[test]
    fn regex_subject_simple() {
        let s = "feat: A new feature";
        let caps = REGEX_SUBJECT.captures(s).unwrap();
        // eprintln!("{caps:#?}");
        assert_eq!(caps.name("type").unwrap().as_str(), "feat");
        assert!(caps.name("scope").is_none());
        assert!(caps.name("breaking").is_none());
        assert_eq!(caps.name("desc").unwrap().as_str(), "A new feature");
    }

    #[test]
    fn regex_subject_excl() {
        let s = "feat!: A new feature";
        let caps = REGEX_SUBJECT.captures(s).unwrap();
        // eprintln!("{caps:#?}");
        assert_eq!(caps.name("type").unwrap().as_str(), "feat");
        assert!(caps.name("scope").is_none());
        assert_eq!(caps.name("breaking").unwrap().as_str(), "!");
        assert_eq!(caps.name("desc").unwrap().as_str(), "A new feature");
    }

    #[test]
    fn regex_subject_scope() {
        let s = "feat(abc): A new feature";
        let caps = REGEX_SUBJECT.captures(s).unwrap();
        // eprintln!("{caps:#?}");
        assert_eq!(caps.name("type").unwrap().as_str(), "feat");
        assert_eq!(caps.name("scope").unwrap().as_str(), "(abc)");
        assert!(caps.name("breaking").is_none());
        assert_eq!(caps.name("desc").unwrap().as_str(), "A new feature");
    }

    #[test]
    fn regex_subject_scope_excl() {
        let s = "feat(abc)!: A new feature";
        let caps = REGEX_SUBJECT.captures(s).unwrap();
        // eprintln!("{caps:#?}");
        assert_eq!(caps.name("type").unwrap().as_str(), "feat");
        assert_eq!(caps.name("scope").unwrap().as_str(), "(abc)");
        assert_eq!(caps.name("breaking").unwrap().as_str(), "!");
        assert_eq!(caps.name("desc").unwrap().as_str(), "A new feature");
    }
}
