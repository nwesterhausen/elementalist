//! # Elementalist Shared Game Library
//!
//! This library contains all the shared game logic for the Elementalist game.
//!
//! ## Fully Generic Components
//!
//! These components are fully generic and can be used in any game. (They are the
//! base components for the Elementalist game.)
//!
//! * [`Attribute`] - An attribute has a current and max value. (e.g. health, mana, etc.)
//! * [`Stat`] - Contains a floating point value that can be used to represent a stat.
//! * [`StatBonus`] - Contains a percentage value with calculations for adding or removing
//! points or percentages from the value. (e.g. +10% health, -5% damage, etc.)
//! * [`Volume`] - Volume component with a "muted" flag.
//! * [`Xp`] - Experience component with a level and experience points, uses a custom
//! experience curve.
//!
//! ## Elementalist Game Components
//!
//! These components are specific to the Elementalist game, and are used by at least
//! two different systems. (If any component would only be used by one system, it is likely
//! just in that system's module, and not in this library.)
//!

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
