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

/// Contains the data loader for the game library, allows for loading in data files from disk.
pub mod data_loader;

mod attribute;
mod enums;
mod experience;
mod skill;
mod spell_data;
mod stat;
mod stat_bonus;
mod stat_effect;
mod volume;

pub use attribute::Attribute;
pub use data_loader::events::*;
pub use enums::*;
pub use experience::Xp;
pub use skill::SkillTrack;
pub use spell_data::SpellData;
pub use stat::Stat;
pub use stat_bonus::StatBonus;
pub use stat_effect::StatEffect;
pub use volume::Volume;

/// Temporary spell enum, will be replaced with a spell data file.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    bevy::ecs::component::Component,
)]
pub enum Spell {
    /// Firebolt spell.
    Firebolt,
}

impl Spell {
    /// Get a list of all the spells in the game.
    pub fn variants() -> Vec<Spell> {
        vec![Spell::Firebolt]
    }
}
