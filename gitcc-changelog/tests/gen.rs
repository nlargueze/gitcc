//! Changelog/Release generation

use gitcc_changelog::{Changelog, Release, Section, TEMPLATE_CHANGELOG_STD, TEMPLATE_RELEASE_STD};
use time::{macros::datetime, OffsetDateTime};

#[test]
fn gen_changelog() {
    let changelog = Changelog {
        releases: vec![
            Release {
                version: "Unreleased".to_string(),
                date: OffsetDateTime::now_utc(),
                url: None,
                sections: vec![
                    Section {
                        label: "New features".to_string(),
                        items: vec![
                            "changelog #1".to_string(),
                            "changelog #2".to_string(),
                            "changelog #3".to_string(),
                        ],
                    },
                    Section {
                        label: "Fixes".to_string(),
                        items: vec![
                            "fix #1".to_string(),
                            "fix #2".to_string(),
                            "fix #3".to_string(),
                        ],
                    },
                ],
            },
            Release {
                version: "v0.0.1".to_string(),
                date: datetime!(2021-01-01 13:00:55 UTC),
                url: Some("https://github.com/gitcc/release/v0.0.1".to_string()),
                sections: vec![
                    Section {
                        label: "New features".to_string(),
                        items: vec![
                            "changelog #1".to_string(),
                            "changelog #2".to_string(),
                            "changelog #3".to_string(),
                        ],
                    },
                    Section {
                        label: "Fixes".to_string(),
                        items: vec![
                            "fix #1".to_string(),
                            "fix #2".to_string(),
                            "fix #3".to_string(),
                        ],
                    },
                ],
            },
        ],
    };

    let changelog_str = changelog.render(TEMPLATE_CHANGELOG_STD).unwrap();
    eprintln!("{changelog_str}");
}

#[test]
fn gen_release() {
    let release = Release {
        version: "v0.0.1".to_string(),
        date: datetime!(2021-01-01 13:00:55 UTC),
        url: Some("https://github.com/gitcc/release/v0.0.1".to_string()),
        sections: vec![
            Section {
                label: "New features".to_string(),
                items: vec![
                    "changelog #1".to_string(),
                    "changelog #2".to_string(),
                    "changelog #3".to_string(),
                ],
            },
            Section {
                label: "Fixes".to_string(),
                items: vec![
                    "fix #1".to_string(),
                    "fix #2".to_string(),
                    "fix #3".to_string(),
                ],
            },
        ],
    };

    let release_str = release.render(TEMPLATE_RELEASE_STD).unwrap();
    eprintln!("{release_str}");
}
