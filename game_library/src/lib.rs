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
pub mod spells;
pub mod state;

mod acceleration;
mod attribute;
mod biome;
mod camera_scale;
mod cursor_position;
mod depth_2d;
mod experience;
mod health;
mod mana;
mod markers_to_biomes;
mod movement_bundle;
mod noise;
mod particle;
mod physics;
mod realm_data;
mod schedule;
mod shared_traits;
mod simple_object;
mod skill;
mod stat;
mod stat_bonus;
mod stat_bundle;
mod stat_effect;
mod tileset;
mod volume;

pub use acceleration::Acceleration;
pub use attribute::Attribute;
pub use biome::BiomeData;
pub use camera_scale::CameraScaleLevel;
pub use cursor_position::CursorPosition;
pub use depth_2d::{Layer, LayerPlugin};
pub use experience::Xp;
pub use health::Health;
pub use mana::Mana;
pub use markers_to_biomes::MarkersToBiomes;
pub use movement_bundle::MovementBundle;
pub use noise::GeneratedMaps;
pub use noise::GenerationSeed;
pub use noise::NoisePlugin;
pub use physics::PhysicsPlugin;
pub use realm_data::Realm;
pub use schedule::*;
pub use shared_traits::InternalId;
pub use simple_object::SimpleObject;
pub use skill::Skills;
pub use stat::Stat;
pub use stat_bonus::StatBonus;
pub use stat_bundle::StatBundle;
pub use stat_effect::StatEffect;
pub use tileset::Tileset;
pub use volume::Volume;
