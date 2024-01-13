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
mod enums;
mod experience;
mod skill;
mod stat;
mod stat_bonus;
mod volume;

pub use attribute::Attribute;
pub use enums::*;
pub use experience::Xp;
pub use skill::SkillTrack;
pub use stat::Stat;
pub use stat_bonus::StatBonus;
pub use volume::Volume;
