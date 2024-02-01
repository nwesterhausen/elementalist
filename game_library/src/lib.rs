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
//! ## Elementalist Game Components/Resources/Events/Enums
//!
//! The remaining items are specific to the Elementalist game.
//!

pub mod colors;
pub mod data_loader;
pub mod enums;
pub mod events;
pub mod font_resource;
pub mod math;
pub mod menu_helper;
pub mod progress_bar;
pub mod settings;

mod acceleration;
mod attribute;
mod camera_scale;
mod cursor_position;
mod experience;
mod health;
mod mana;
mod movement_bundle;
mod shared_traits;
mod skill;
mod spell_bundle;
mod spell_choices;
mod spell_data;
mod spell_lifetime;
mod stat;
mod stat_bonus;
mod stat_bundle;
mod stat_effect;
mod tileset;
mod velocity;
mod volume;

pub use acceleration::Acceleration;
pub use attribute::Attribute;
pub use camera_scale::CameraScaleLevel;
pub use cursor_position::CursorPosition;
pub use data_loader::events::*;
pub use experience::Xp;
pub use health::Health;
pub use mana::Mana;
pub use movement_bundle::MovementBundle;
pub use shared_traits::InternalId;
pub use skill::SkillTrack;
pub use spell_bundle::SpellBundle;
pub use spell_choices::SpellChoices;
pub use spell_data::SpellData;
pub use spell_lifetime::SpellLifetime;
pub use stat::Stat;
pub use stat_bonus::StatBonus;
pub use stat_bundle::StatBundle;
pub use stat_effect::StatEffect;
pub use tileset::Tileset;
pub use velocity::Velocity;
pub use volume::Volume;
