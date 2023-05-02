//! Core functionalities

mod changelog;
mod commit;
mod config;
mod error;

pub use changelog::*;
pub use commit::*;
pub use config::*;
pub use error::*;

pub use gitcc_changelog::TEMPLATE_CHANGELOG_STD;
pub use gitcc_convco::{ConvcoMessage, StringExt};
pub use time;
