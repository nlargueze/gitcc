//! Extra git commands to manage repos commit and release process.
//!
//! The crate provides a number of executables to:
//! - Commit using conventional commits.
//! - Determine the next version following the conventional commit guidelines.
//! - Bump the version in the code package manifest.
//! - Create a CHANGELOG.
//! - Perform the release process.

#![deny(missing_docs)]

pub mod changelog;
pub mod commands;
pub mod config;
pub mod conventional;
pub mod error;
pub mod git;
pub mod hooks;
pub mod utils;
pub mod version;
