//! Spell data describes how a spell works and how it should be displayed.
//!
//! You can describe spell data using YAML or JSON using the schema:
//!
//! ```yaml
//! # $schema: "https://schemas.nwest.one/games/elementalist/spell.json"
//! ```
use serde_default_utils::{default_i32, default_usize};
use std::{any::Any, hash::Hash};

use crate::{
    data_loader::DataFile,
    enums::{CastCategory, CastSlot, CastType, GameSystem, MagicType, Skill, SpellCollision},
    InternalId, StatEffect,
};

/// Details about a spell.
///
/// Describes in detail how a spell works and how it should be displayed.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
    pub spell_tier: usize,
    /// The type of magic the spell uses.
    pub magic: MagicType,
    /// The slot the spell can be cast from.
    pub cast_slot: CastSlot,
    /// The type of collision the spell has.
    #[serde(default = "spell_defaults::collision")]
    pub collision: SpellCollision,
    /// The type of cast the spell has.
    #[serde(default = "spell_defaults::cast_type")]
    pub cast_type: CastType,
    /// How the spell is targeted
    #[serde(default = "spell_defaults::cast_category")]
    pub cast_category: CastCategory,

    // #### SPELL ICONS ####
    /// The path to the icon for the spell (relative to the game's asset directory).
    #[serde(default = "spell_defaults::placeholder_png_path")]
    pub icon_tileset: String,
    /// The index of the spell's icon in the tileset.
    #[serde(default = "default_usize::<0>")]
    pub icon_index: usize,
    /// The path to the sprite tileset for the spell (relative to the game's asset directory).
    #[serde(default = "spell_defaults::placeholder_png_path")]
    pub sprite_tileset: String,
    /// The index of the spell's sprite in the tileset.
    #[serde(default = "default_usize::<0>")]
    pub sprite_index: usize,

    // #### SPELL BASE STATS ####
    /// The cooldown of the spell in seconds
    #[serde(default = "spell_defaults::spell_cooldown")]
    pub cooldown: f32,
    /// The cast time of the spell in seconds
    #[serde(default = "spell_defaults::spell_cast_time")]
    pub cast_time: f32,
    /// The mana cost of the spell (mana is an integer value)
    #[serde(default = "default_usize::<0>")]
    pub mana_cost: usize,
    /// The range of the spell in centimeters.
    #[serde(default = "spell_defaults::spell_range")]
    pub range: f32,
    /// The speed of the spell in meters per second.
    #[serde(default = "spell_defaults::spell_speed")]
    pub speed: f32,
    /// The duration of the spell in seconds.
    #[serde(default = "spell_defaults::spell_duration")]
    pub duration: f32,
    /// The base damage of the spell, 0 if the spell does not deal damage.
    #[serde(default = "default_i32::<0>")]
    pub damage: i32,
    /// The base healing of the spell, 0 if the spell does not heal.
    #[serde(default = "default_i32::<0>")]
    pub healing: i32,
    /// Radius of the spell "detonation" in centimeters.
    ///
    /// Spells which target the ground use this value to determine the radius of the area of effect.
    ///
    /// Spells which are cone or line shaped use this value to determine the width of the cone or line.
    #[serde(default = "default_i32::<0>")]
    pub radius: i32,
    /// The angle of the spell in degrees.
    ///
    /// Spells which are cone shaped use this value to determine the angle of the cone.
    #[serde(default = "default_i32::<0>")]
    pub angle: i32,

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
        self.name.hash(state);
        self.spell_tier.hash(state);
        self.magic.hash(state);
        self.cast_slot.hash(state);
    }
}

impl InternalId for SpellData {
    /// Update the spell's internal ID.
    fn update_internal_id(&mut self) {
        self.internal_id = Some(self.get_internal_id());
    }
    /// Get the spell's internal ID.
    #[must_use]
    fn get_internal_id(&self) -> String {
        if self.internal_id.is_some() {
            let id = self.internal_id.clone().unwrap_or_default();
            if !id.is_empty() {
                return id;
            }
        }

        format!(
            "{}{}{}{}",
            self.name.replace(' ', ""),
            self.spell_tier,
            self.magic,
            self.cast_slot
        )
    }
}

pub trait KnownCastSlot {
    fn cast_slot(&self) -> CastSlot;
}

impl KnownCastSlot for SpellData {
    fn cast_slot(&self) -> CastSlot {
        self.cast_slot
    }
}

impl SpellData {
    /// Returns the skill that the spell uses.
    #[must_use]
    pub fn skill(&self) -> Skill {
        self.magic.into()
    }
    /// Get the spell's sprite as a texture atlas sprite.
    #[must_use]
    pub fn texture_atlas_index(&self) -> bevy::sprite::TextureAtlasSprite {
        bevy::sprite::TextureAtlasSprite::new(self.sprite_index)
    }
}

/// #### DEFAULTS FOR SERDE ####
mod spell_defaults {
    use crate::enums::{CastCategory, CastType, SpellCollision};

    pub(super) const fn collision() -> SpellCollision {
        SpellCollision::Point
    }
    pub(super) const fn cast_type() -> CastType {
        CastType::Instant
    }
    pub(super) const fn cast_category() -> CastCategory {
        CastCategory::Projectile
    }
    pub(super) fn placeholder_png_path() -> String {
        "placeholder.png".to_string()
    }
    pub(super) const fn spell_cooldown() -> f32 {
        1.0
    }
    pub(super) const fn spell_cast_time() -> f32 {
        0.0
    }
    pub(super) const fn spell_range() -> f32 {
        5.0
    }
    pub(super) const fn spell_speed() -> f32 {
        1.0
    }
    pub(super) const fn spell_duration() -> f32 {
        5.0
    }
}

impl<D: Hash + InternalId + 'static> TryInto<SpellData> for DataFile<D> {
    type Error = ();

    fn try_into(self) -> Result<SpellData, Self::Error> {
        if self.header.system != GameSystem::Spell {
            return Err(());
        }

        (&self.data as &dyn Any)
            .downcast_ref::<SpellData>()
            .cloned()
            .ok_or(())
    }
}

impl<D: Hash + InternalId + 'static> TryFrom<&DataFile<D>> for SpellData {
    type Error = ();

    fn try_from(data_file: &DataFile<D>) -> Result<Self, Self::Error> {
        if data_file.header.system != GameSystem::Tileset {
            return Err(());
        }

        (&data_file.data as &dyn Any)
            .downcast_ref::<Self>()
            .cloned()
            .ok_or(())
    }
}

impl Default for SpellData {
    fn default() -> Self {
        Self {
            internal_id: None,
            name: "Unnamed Spell".to_string(),
            description: "No description provided.".to_string(),
            long_description: String::new(),
            spell_tier: 0,
            magic: MagicType::Arcane,
            cast_slot: CastSlot::Primary,
            collision: SpellCollision::Point,
            cast_type: CastType::Instant,
            cast_category: CastCategory::Projectile,
            icon_tileset: spell_defaults::placeholder_png_path(),
            icon_index: 0,
            sprite_tileset: spell_defaults::placeholder_png_path(),
            sprite_index: 0,
            cooldown: spell_defaults::spell_cooldown(),
            cast_time: spell_defaults::spell_cast_time(),
            mana_cost: 0,
            range: spell_defaults::spell_range(),
            speed: spell_defaults::spell_speed(),
            duration: spell_defaults::spell_duration(),
            damage: 0,
            healing: 0,
            radius: 0,
            angle: 0,
            buffs: Vec::new(),
            debuffs: Vec::new(),
        }
    }
}
