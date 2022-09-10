//! Testing of conventional commits

use gitext::conventional::ConventionalCommitMessage;
use indoc::indoc;

#[test]
fn oneliner_no_scope() {
    let commit = ConventionalCommitMessage {
        r#type: "fix".to_string(),
        scope: None,
        subject: "commit subject".to_string(),
        body: None,
        breaking_change: None,
        closed_issues: None,
    };

    let msg = indoc!("fix: commit subject");

    assert_eq!(commit.to_string(), msg);
}

#[test]
fn oneliner_scope() {
    let commit = ConventionalCommitMessage {
        r#type: "fix".to_string(),
        scope: Some("myscope".to_string()),
        subject: "commit subject".to_string(),
        body: None,
        breaking_change: None,
        closed_issues: None,
    };

    let msg = indoc!("fix(myscope): commit subject");

    assert_eq!(commit.to_string(), msg);
}

#[test]
fn oneliner_scope_breaking_change() {
    let commit = ConventionalCommitMessage {
        r#type: "fix".to_string(),
        scope: Some("myscope".to_string()),
        subject: "commit subject".to_string(),
        body: None,
        breaking_change: Some("breaking change".to_string()),
        closed_issues: None,
    };

    let msg = indoc!(
        "fix(myscope)!: commit subject
    
    BREAKING CHANGE: breaking change"
    );

    assert_eq!(commit.to_string(), msg);
}

#[test]
fn body() {
    let commit = ConventionalCommitMessage {
        r#type: "fix".to_string(),
        scope: Some("myscope".to_string()),
        subject: "commit subject".to_string(),
        body: Some("Commit body\nAnother body line".to_string()),
        breaking_change: None,
        closed_issues: None,
    };

    let msg = indoc!(
        "fix(myscope): commit subject
    
    Commit body
    Another body line"
    );

    assert_eq!(commit.to_string(), msg);
}

#[test]
fn body_with_issues() {
    let commit = ConventionalCommitMessage {
        r#type: "fix".to_string(),
        scope: Some("myscope".to_string()),
        subject: "commit subject".to_string(),
        body: Some("Commit body\nAnother body line".to_string()),
        breaking_change: None,
        closed_issues: Some(vec![1, 2]),
    };

    let msg = indoc!(
        "fix(myscope): commit subject
    
    Commit body
    Another body line
    
    Closes #1
    Closes #2"
    );

    assert_eq!(commit.to_string(), msg);
}

#[test]
fn body_with_change_and_issues() {
    let commit = ConventionalCommitMessage {
        r#type: "fix".to_string(),
        scope: Some("myscope".to_string()),
        subject: "commit subject".to_string(),
        body: Some("Commit body\nAnother body line".to_string()),
        breaking_change: Some("A breaking change".to_string()),
        closed_issues: Some(vec![1, 2]),
    };

    let msg = indoc!(
        "fix(myscope)!: commit subject
    
    Commit body
    Another body line
    
    BREAKING CHANGE: A breaking change
    Closes #1
    Closes #2"
    );

    assert_eq!(commit.to_string(), msg);
}
