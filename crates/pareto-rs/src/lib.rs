//! ParetoOptimal cost engine — pure pricing & routing logic.
//!
//! No I/O, no CLI, no external API calls. Just pure business logic.

pub mod models;
pub mod cost;
pub mod pricing;
pub mod utils;
pub mod format;

pub use models::*;
pub use cost::*;
pub use pricing::*;
