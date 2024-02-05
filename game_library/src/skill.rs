//! Skills are used to track a meta-progression of a player's abilities.
//!
//! Skills keep track of both the experience points and the level of the skill.
//!
//! Skills are able to be leveled up, and have no level cap but do have a
//! soft cap that makes it harder to level up the higher the skill is (this is
//! automatically handled by the [`game_library::Xp`] component).

use bevy::{
    ecs::{component::Component, system::Resource},
    reflect::Reflect,
    utils::HashMap,
};
use bevy_inspector_egui::inspector_options::{InspectorOptions, ReflectInspectorOptions};
use serde::{Deserialize, Serialize};

use crate::{enums::Skill, Xp};

/// Skills are used to track a meta-progression of a player's abilities.
///
/// Skills are able to be leveled up, and have no level cap but do have a
/// soft cap that makes it harder to level up the higher the skill is.
///
/// Skill levels allocate points to the skill's attributes, which are
/// used when playing the game, making some schools of magic more powerful
/// than others.
///
/// Skills are also used to gate spells, so that a player must have a certain
/// level in a skill before they can unlock a spell. All of that is built on top
/// of this skill system.
#[derive(
    Resource,
    Component,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    Reflect,
    InspectorOptions,
)]
#[reflect(InspectorOptions)]
#[allow(clippy::module_name_repetitions)]
pub struct Skills {
    /// Skill tracks for each of the different skills.
    pub tracks: HashMap<Skill, Xp>,
}

impl Skills {
    /// Get xp for a skill.
    #[must_use]
    pub fn get_xp(&self, skill: Skill) -> Option<&Xp> {
        self.tracks.get(&skill)
    }
    /// Get the level for a skill.
    #[must_use]
    pub fn get_level(&self, skill: Skill) -> Option<u32> {
        self.get_xp(skill).map(|xp| xp.current_level)
    }
    /// Get the percentage to the next level for a skill.
    #[must_use]
    pub fn get_percentage_to_next_level(&self, skill: Skill) -> Option<f32> {
        self.get_xp(skill).map(Xp::next_level_progress)
    }
    /// Add xp to a skill.
    pub fn add_xp(&mut self, skill: Skill, xp: u32) {
        if let Some(skill_xp) = self.tracks.get_mut(&skill) {
            skill_xp.add(xp);
        }
    }
    /// Level-up a skill.
    pub fn level_up(&mut self, skill: Skill) {
        if let Some(skill_xp) = self.tracks.get_mut(&skill) {
            skill_xp.level_up();
        }
    }
}
