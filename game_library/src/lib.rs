//! # Shared Game Library
//!
//! The library contains generic structs that aren't specific to the game.

#![warn(
    missing_docs,
    unreachable_code,
    unreachable_patterns,
    clippy::unwrap_used,
    clippy::expect_used
)]
#![deny(unsafe_code)]

mod attribute;
mod volume;

pub use attribute::Attribute;
pub use volume::Volume;
