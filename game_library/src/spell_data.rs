//! Spell data describes how a spell works and how it should be displayed.
//!
//! You can describe spell data using YAML or JSON using the schema:
//!
//! ```yaml
//! # $schema: "https://schemas.nwest.one/games/elementalist/spell.json"
//! ```
use serde_default_utils::*;
use std::hash::Hash;

use crate::{CastCategory, CastSlot, CastType, MagicType, Skill, SpellCollision, StatEffect};

/// Details about a spell.
///
/// Describes in detail how a spell works and how it should be displayed.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpellData {
    /// The internal ID of the spell.
    pub internal_id: Option<String>,
    /// The name of the spell.
    pub name: String,
    /// A short description of the spell.
    pub description: String,
    /// A longer description of the spell.
    #[serde(default = "String::new")]
    pub long_description: String,
    /// The tier of the spell (0-9 officially).
    pub spell_tier: u8,
    /// The type of magic the spell uses.
    pub magic: MagicType,
    /// The slot the spell can be cast from.
    pub cast_slot: CastSlot,
    /// The type of collision the spell has.
    #[serde(default = "default_spell_collision")]
    pub collision: SpellCollision,
    /// The type of cast the spell has.
    #[serde(default = "default_cast_type")]
    pub cast_type: CastType,
    /// How the spell is targeted
    #[serde(default = "default_cast_category")]
    pub cast_category: CastCategory,

    // #### SPELL ICONS ####
    /// The path to the icon for the spell (relative to the game's asset directory).
    #[serde(default = "placeholder_png_path")]
    pub icon: String,
    /// The path to the sprite for the spell (relative to the game's asset directory).
    #[serde(default = "placeholder_png_path")]
    pub sprite: String,

    // #### SPELL BASE STATS ####
    /// The cooldown of the spell in centiseconds (hundredths of a second, e.g. 2.05 would be 205).
    ///
    /// If not specified the default value is 100 (1 second).
    #[serde(default = "default_u32::<100>")]
    pub cooldown: u32,
    /// The cast time of the spell in centiseconds (hundredths of a second, e.g. 1.0 would be 100).
    #[serde(default = "default_u32::<0>")]
    pub cast_time: u32,
    /// The mana cost of the spell (mana is an integer value)
    #[serde(default = "default_u8::<0>")]
    pub mana_cost: u8,
    /// The range of the spell in centimeters.
    ///
    /// If not specified the default value is 500 (5m)
    #[serde(default = "default_u32::<500>")]
    pub range: u32,
    /// The speed of the spell in centimeters per second.
    ///
    /// If not specified the default value is 100 (1m/s)
    #[serde(default = "default_u32::<100>")]
    pub speed: u32,
    /// The duration of the spell in centiseconds (hundredths of a second, e.g. 1.0 would be 100).
    ///
    /// If not specified the default value is 500 (5 seconds)
    #[serde(default = "default_u32::<500>")]
    pub duration: u32,
    /// The base damage of the spell, 0 if the spell does not deal damage.
    #[serde(default = "default_u32::<0>")]
    pub damage: u32,
    /// The base healing of the spell, 0 if the spell does not heal.
    #[serde(default = "default_u32::<0>")]
    pub healing: u32,
    /// Radius of the spell "detonation" in centimeters.
    ///
    /// Spells which target the ground use this value to determine the radius of the area of effect.
    ///
    /// Spells which are cone or line shaped use this value to determine the width of the cone or line.
    #[serde(default = "default_u32::<0>")]
    pub radius: u32,
    /// The angle of the spell in degrees.
    ///
    /// Spells which are cone shaped use this value to determine the angle of the cone.
    #[serde(default = "default_u32::<0>")]
    pub angle: u32,

    // #### SPELL EFFECTS ####
    /// Buffs that the spell can apply to the caster or to the target.
    #[serde(default = "Vec::new")]
    pub buffs: Vec<StatEffect>,
    /// Debuffs that the spell can apply to the caster or to the target.
    #[serde(default = "Vec::new")]
    pub debuffs: Vec<StatEffect>,
}

impl Hash for SpellData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if let Some(internal_id) = &self.internal_id {
            internal_id.hash(state);
            return;
        }
        self.name.hash(state);
        self.description.hash(state);
        self.spell_tier.hash(state);
        self.radius.hash(state);
    }
}

impl SpellData {
    /// Returns the skill that the spell uses.
    pub fn skill(&self) -> Skill {
        self.magic.into()
    }
}

// #### DEFAULTS FOR SERDE ####
fn default_spell_collision() -> SpellCollision {
    SpellCollision::Point
}
fn default_cast_type() -> CastType {
    CastType::Instant
}
fn default_cast_category() -> CastCategory {
    CastCategory::Projectile
}
fn placeholder_png_path() -> String {
    "placeholder.png".to_string()
}
