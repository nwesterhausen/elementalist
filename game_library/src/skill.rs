use bevy::ecs::{component::Component, system::Resource};
use serde::{Deserialize, Serialize};

use crate::{Skill, Xp};

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
#[derive(Resource, Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SkillTrack {
    /// The skill the this is tracking
    pub skill: Skill,
    /// The experience points the player has in this skill
    pub xp: Xp,
}
