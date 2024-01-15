//! Spell data describes how a spell works and how it should be displayed.
//!
//! You can describe spell data using YAML or JSON using the schema:
//!
//! ```yaml
//! # $schema: "https://schemas.nwest.one/games/elementalist/spell.json"
//! ```

use crate::{CastCategory, CastSlot, CastType, MagicType, Skill, SpellCollision, StatEffect};

/// Details about a spell.
///
/// Describes in detail how a spell works and how it should be displayed.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpellData {
    /// The internal ID of the spell.
    pub internal_id: String,
    /// The name of the spell.
    pub name: String,
    /// A short description of the spell.
    pub description: String,
    /// A longer description of the spell.
    pub long_description: String,
    /// The tier of the spell (0-9 officially).
    pub spell_tier: u8,
    /// The type of magic the spell uses.
    pub magic: MagicType,
    /// The skill the spell uses.
    pub skill: Skill,
    /// The slot the spell can be cast from.
    pub cast_slot: CastSlot,
    /// The type of collision the spell has.
    pub collision: SpellCollision,
    /// The type of cast the spell has.
    pub cast_type: CastType,
    /// How the spell is targeted
    pub cast_category: CastCategory,

    // #### SPELL ICONS ####
    /// The path to the icon for the spell (relative to the game's asset directory).
    pub icon: String,
    /// The path to the sprite for the spell (relative to the game's asset directory).
    pub sprite: String,

    // #### SPELL BASE STATS ####
    /// The cooldown of the spell in centiseconds (hundredths of a second, e.g. 2.05 would be 205).
    pub cooldown: u32,
    /// The cast time of the spell in centiseconds (hundredths of a second, e.g. 1.0 would be 100).
    pub cast_time: u32,
    /// The mana cost of the spell (mana is an integer value)
    pub mana_cost: u8,
    /// The range of the spell in centimeters.
    pub range: u32,
    /// The speed of the spell in centimeters per second.
    pub speed: u32,
    /// The duration of the spell in centiseconds (hundredths of a second, e.g. 1.0 would be 100).
    pub duration: u32,
    /// The base damage of the spell, 0 if the spell does not deal damage.
    pub damage: u32,
    /// The base healing of the spell, 0 if the spell does not heal.
    pub healing: u32,
    /// Radius of the spell "detonation" in centimeters.
    ///
    /// Spells which target the ground use this value to determine the radius of the area of effect.
    ///
    /// Spells which are cone or line shaped use this value to determine the width of the cone or line.
    pub radius: u32,
    /// The angle of the spell in degrees.
    ///
    /// Spells which are cone shaped use this value to determine the angle of the cone.
    pub angle: u32,

    // #### SPELL EFFECTS ####
    /// Buffs that the spell can apply to the caster or to the target.
    pub buffs: Vec<StatEffect>,
    /// Debuffs that the spell can apply to the caster or to the target.
    pub debuffs: Vec<StatEffect>,
}
