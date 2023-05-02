//! Changelog
//!
//! The changelog format is defined in https://keepachangelog.com/en/1.0.0/.

use handlebars::Handlebars;
use serde::{Serialize, Serializer};
use time::{macros::format_description, OffsetDateTime};

/// Base changelog template
pub const TEMPLATE_CHANGELOG_STD: &str = include_str!("tpl/changelog.hbs");

/// Base release template
pub const TEMPLATE_RELEASE_STD: &str = include_str!("tpl/release.hbs");

/// Changelog
///
/// The generic `T` is the "form" of the commit
#[derive(Debug, Serialize)]
pub struct Changelog {
    /// Releases (latest first)
    pub releases: Vec<Release>,
}

/// Changelog release
#[derive(Debug, Serialize)]
pub struct Release {
    /// Version (v0.0.1, Unreleased, ...)
    pub version: String,
    /// Release date
    #[serde(serialize_with = "serialize_date")]
    pub date: OffsetDateTime,
    /// A link to the release
    ///
    /// [Unreleased]: https://github.com/olivierlacan/keep-a-changelog/compare/v1.0.0...HEAD
    /// [1.0.0]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.0.2...v0.0.1
    pub url: Option<String>,
    /// Sections
    pub sections: Vec<Section>,
}

/// Serializes a date
fn serialize_date<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let format = format_description!("[year]-[month]-[day]");
    date.format(&format).unwrap().serialize(serializer)
}

/// Changelog release section
#[derive(Debug, Serialize)]
pub struct Section {
    /// Section label
    pub label: String,
    /// Section items
    pub items: Vec<String>,
}

impl PartialEq for Section {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

/// Changelog error
#[derive(Debug, thiserror::Error)]
#[error("Changelog error:{0}")]
pub struct Error(String);

impl Changelog {
    /// Generates the change log
    pub fn render(&self, template: &str) -> Result<String, Error> {
        let handlebars = Handlebars::new();
        handlebars
            .render_template(template, &self)
            .map_err(|err| Error(err.to_string()))
    }
}

impl Release {
    /// Generates the release note
    pub fn render(&self, template: &str) -> Result<String, Error> {
        let handlebars = Handlebars::new();
        handlebars
            .render_template(template, &self)
            .map_err(|err| Error(err.to_string()))
    }
}
