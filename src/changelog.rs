//! Changelog

use indoc::indoc;
use serde::{Deserialize, Serialize};

/// Changelog configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeLogConfig {
    /// Commit types included in the changelog
    pub incl_types: Vec<String>,
}

impl Default for ChangeLogConfig {
    fn default() -> Self {
        let incl_types = vec![
            "feat".to_string(),
            "fix".to_string(),
            "docs".to_string(),
            "perf".to_string(),
            "test".to_string(),
            "build".to_string(),
            "ci".to_string(),
            "cd".to_string(),
            "chore".to_string(),
        ];

        Self { incl_types }
    }
}

/// Changelog template
const CHANGELOG_TEMPLATE: &str = indoc!(
    "# Changelog
    
    All notable changes to this project will be documented in this file.

    {{#each releases}}
    ## [{{this.version}}] - {{this.date}}
    {{#if this.history_url}}

    {{this.history_url}}
    {{/if}}

    {{#each this.groups}}
    ### {{this.title}}

    {{#each this.commits}}
    - {{this.prefix}}{{this.subject}} {{this.commit_link}}
    {{/each}}

    {{/each}}
    {{/each}}"
);
