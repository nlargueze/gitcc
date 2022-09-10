//! Testing of conventional commits parsing

use gitext::{config::Config, conventional::ConventionalCommitMessage};
use indoc::indoc;

#[test]
fn oneliner_simple() {
    let msg = indoc!("fix: commit subject");

    let commit =
        ConventionalCommitMessage::parse(msg, &Config::default().valid_commit_types()).unwrap();

    assert_eq!(
        commit,
        ConventionalCommitMessage {
            r#type: "fix".to_string(),
            scope: None,
            subject: "commit subject".to_string(),
            body: None,
            breaking_change: None,
            closed_issues: None
        }
    )
}

#[test]
fn oneliner_scope() {
    let msg = indoc!("fix(myscope): commit subject");

    let commit =
        ConventionalCommitMessage::parse(msg, &Config::default().valid_commit_types()).unwrap();

    assert_eq!(
        commit,
        ConventionalCommitMessage {
            r#type: "fix".to_string(),
            scope: Some("myscope".to_string()),
            subject: "commit subject".to_string(),
            body: None,
            breaking_change: None,
            closed_issues: None
        }
    )
}

#[test]
fn oneliner_breaking() {
    let msg = indoc!("fix!: commit subject");

    let commit =
        ConventionalCommitMessage::parse(msg, &Config::default().valid_commit_types()).unwrap();

    assert_eq!(
        commit,
        ConventionalCommitMessage {
            r#type: "fix".to_string(),
            scope: None,
            subject: "commit subject".to_string(),
            body: None,
            breaking_change: Some("".to_string()),
            closed_issues: None
        }
    )
}

#[test]
fn oneliner_scope_breaking() {
    let msg = indoc!("fix(myscope)!: commit subject");

    let commit =
        ConventionalCommitMessage::parse(msg, &Config::default().valid_commit_types()).unwrap();

    assert_eq!(
        commit,
        ConventionalCommitMessage {
            r#type: "fix".to_string(),
            scope: Some("myscope".to_string()),
            subject: "commit subject".to_string(),
            body: None,
            breaking_change: Some("".to_string()),
            closed_issues: None
        }
    )
}

#[test]
fn body_simple() {
    let msg = indoc!(
        "fix: commit subject

    commit body"
    );

    let commit =
        ConventionalCommitMessage::parse(msg, &Config::default().valid_commit_types()).unwrap();

    assert_eq!(
        commit,
        ConventionalCommitMessage {
            r#type: "fix".to_string(),
            scope: None,
            subject: "commit subject".to_string(),
            body: Some("commit body".to_string()),
            breaking_change: None,
            closed_issues: None
        }
    )
}

#[test]
fn body_multiline_simple() {
    let msg = indoc!(
        "fix: commit subject

    commit body
    commit body line 2"
    );

    let commit =
        ConventionalCommitMessage::parse(msg, &Config::default().valid_commit_types()).unwrap();

    assert_eq!(
        commit,
        ConventionalCommitMessage {
            r#type: "fix".to_string(),
            scope: None,
            subject: "commit subject".to_string(),
            body: Some("commit body\ncommit body line 2".to_string()),
            breaking_change: None,
            closed_issues: None
        }
    )
}

#[test]
fn body_multiline_breaking_change_no_issues() {
    let msg = indoc!(
        "fix: commit subject

    commit body
    commit body line 2
    
    BREAKING CHANGE: this is a breaking change
    on several line"
    );

    let commit =
        ConventionalCommitMessage::parse(msg, &Config::default().valid_commit_types()).unwrap();

    assert_eq!(
        commit,
        ConventionalCommitMessage {
            r#type: "fix".to_string(),
            scope: None,
            subject: "commit subject".to_string(),
            body: Some("commit body\ncommit body line 2".to_string()),
            breaking_change: Some("this is a breaking change\non several line".to_string()),
            closed_issues: None
        }
    )
}

#[test]
fn body_multiline_breaking_change_issues() {
    let msg = indoc!(
        "fix: commit subject

    commit body
    commit body line 2
    
    BREAKING CHANGE: this is a breaking change
    on several line
    Closes #1"
    );

    let commit =
        ConventionalCommitMessage::parse(msg, &Config::default().valid_commit_types()).unwrap();

    assert_eq!(
        commit,
        ConventionalCommitMessage {
            r#type: "fix".to_string(),
            scope: None,
            subject: "commit subject".to_string(),
            body: Some("commit body\ncommit body line 2".to_string()),
            breaking_change: Some("this is a breaking change\non several line".to_string()),
            closed_issues: Some(vec![1])
        }
    )
}

#[test]
fn body_multiline_issues() {
    let msg = indoc!(
        "fix: commit subject

    commit body
    commit body line 2
    
    Closes #1"
    );

    let commit =
        ConventionalCommitMessage::parse(msg, &Config::default().valid_commit_types()).unwrap();

    assert_eq!(
        commit,
        ConventionalCommitMessage {
            r#type: "fix".to_string(),
            scope: None,
            subject: "commit subject".to_string(),
            body: Some("commit body\ncommit body line 2".to_string()),
            breaking_change: None,
            closed_issues: Some(vec![1])
        }
    )
}

#[test]
fn body_multiline_issues_2() {
    let msg = indoc!(
        "fix: commit subject

    commit body
    commit body line 2
    
    Closes #1
    Closes #2"
    );

    let commit =
        ConventionalCommitMessage::parse(msg, &Config::default().valid_commit_types()).unwrap();

    assert_eq!(
        commit,
        ConventionalCommitMessage {
            r#type: "fix".to_string(),
            scope: None,
            subject: "commit subject".to_string(),
            body: Some("commit body\ncommit body line 2".to_string()),
            breaking_change: None,
            closed_issues: Some(vec![1, 2])
        }
    )
}
