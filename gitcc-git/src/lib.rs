//! Git functionalities
//!
//! Under the hood, this crate uses the `git2` crate. It wraps it to provide a higher-level API.

mod commit;
mod config;
mod error;
mod remote;
mod repo;
mod status;
mod tag;

pub use commit::*;
pub use config::*;
pub use error::*;
pub use remote::*;
pub use repo::*;
pub use status::*;
pub use tag::*;
