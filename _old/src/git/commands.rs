//! Wrappers for git commands

pub mod add;
pub mod commit;
pub mod config;
pub mod push;
pub mod status;
pub mod tag;

pub use add::*;
pub use commit::*;
pub use config::*;
pub use push::*;
pub use status::*;
pub use tag::*;
