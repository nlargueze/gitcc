//! Core functionalities

mod changelog;
mod commit;
mod config;
mod error;
mod release;

pub use changelog::*;
pub use commit::*;
pub use config::*;
pub use error::*;
pub use release::*;

pub use gitcc_changelog::TEMPLATE_CHANGELOG_STD;
pub use gitcc_convco::{ConvcoMessage, StringExt};
pub use time;
