//! Defines the `Spell` struct and its methods.

use super::{talisman::SpellTalisman, SpellParticles};
use crate::{
    data_loader::DataFile,
    enums::{CastSlot, CastType, GameSystem, Skill},
    InternalId, StatEffect,
};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use serde_default_utils::default_usize;
use std::{any::Any, hash::Hash};

/// A spell that can be cast by the player.
#[derive(Debug, Clone, Serialize, Deserialize, Reflect, Component, Resource)]
#[serde(rename_all = "camelCase")]
pub struct Spell {
    /// The unique ID of the spell. This is actually in the header of the spell definition file.
    #[serde(default = "String::new")]
    unique_id: String,
    /// The name of the spell.
    #[serde(default = "String::new")]
    name: String,
    /// A short description of the spell.
    #[serde(default = "String::new")]
    description: String,
    /// A longer description, with more details and lore.
    #[serde(default = "String::new")]
    lore: String,
    /// The spell talisman that creates this spell
    #[serde(default = "spell_defaults::talisman")]
    talisman: SpellTalisman,
    /// The skill that the spell uses
    #[serde(default = "spell_defaults::skill")]
    skill: Skill,
    /// The slot that the spell can be cast from
    #[serde(default = "spell_defaults::cast_slot")]
    cast_slot: CastSlot,
    /// The type of casting that the spell uses
    #[serde(default = "spell_defaults::cast_type")]
    cast_type: CastType,

    /// Cooldown time in seconds
    #[serde(default = "spell_defaults::cooldown")]
    cooldown: f32,
    /// Cast time in seconds
    #[serde(default = "spell_defaults::cast_time")]
    cast_time: f32,
    /// Mana cost
    #[serde(default = "spell_defaults::mana_cost")]
    mana_cost: f32,
    /// Range in meters
    #[serde(default = "spell_defaults::range")]
    range: f32,
    /// Speed in meters per second (this is the maximum speed)
    #[serde(default = "spell_defaults::speed")]
    speed: f32,
    /// The acceleration of the spell
    #[serde(default = "spell_defaults::acceleration")]
    acceleration: f32,
    /// The duration of the spell in seconds
    #[serde(default = "spell_defaults::duration")]
    duration: f32,

    /// Base damage of the spell
    #[serde(default = "spell_defaults::base_damage")]
    base_damage: f32,
    /// Base healing of the spell
    #[serde(default = "spell_defaults::base_healing")]
    base_healing: f32,
    /// Buffs applied by the spell
    #[serde(default = "Vec::new")]
    buffs: Vec<StatEffect>,
    /// Debuffs applied by the spell
    #[serde(default = "Vec::new")]
    debuffs: Vec<StatEffect>,

    /// Radius of the spell's area of effect. This affects the collision for `Projectile`, `Cone`, and `Line` spells, while
    /// `AreaOfEffect` spells use this for their area of effect.
    #[serde(default = "spell_defaults::radius")]
    radius: f32,
    /// An angle in radians to affect the area covered by a `Cone` spell.
    #[serde(default = "spell_defaults::angle")]
    angle: f32,

    /// The spell icon's tileset id
    #[serde(default = "String::new")]
    icon_tileset: String,
    /// The spell icon's index in the tileset
    #[serde(default = "default_usize::<0>")]
    icon_index: usize,
    /// The spell sprite's tileset id
    #[serde(default = "String::new")]
    sprite_tileset: String,
    /// The spell sprite's index in the tileset
    #[serde(default = "default_usize::<0>")]
    sprite_index: usize,

    /// The particle effects that the spell creates
    #[serde(default = "Vec::new")]
    particles: Vec<SpellParticles>,
    // The sound effects that the spell creates
    // not implemented yet
}

impl Default for Spell {
    fn default() -> Self {
        Self {
            unique_id: String::new(),
            name: String::from("Default Spell"),
            description: String::from("This is the default spell, has no effects."),
            lore: String::new(),
            talisman: SpellTalisman::default(),
            skill: Skill::Arcanomancy,
            cast_slot: CastSlot::Cantrip,
            cast_type: CastType::Instant,
            cooldown: 0.0,
            cast_time: 0.0,
            mana_cost: 0.0,
            range: 0.0,
            speed: 0.0,
            acceleration: 0.0,
            duration: 0.0,
            base_damage: 0.0,
            base_healing: 0.0,
            buffs: Vec::new(),
            debuffs: Vec::new(),
            radius: 0.0,
            angle: 0.0,
            icon_tileset: String::new(),
            icon_index: 0,
            sprite_tileset: String::new(),
            sprite_index: 0,
            particles: Vec::new(),
        }
    }
}

impl Hash for Spell {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.unique_id.hash(state);
        self.skill.hash(state);
        self.cast_slot.hash(state);
    }
}

impl InternalId for Spell {
    fn update_internal_id(&mut self) {
        if self.unique_id.is_empty() {
            warn!(
                "Spell '{}' has no unique_id, using name as unique_id.",
                self.name
            );
            // Replace spaces with underscores, and make it lowercase
            self.unique_id = self.name.to_lowercase().replace(' ', "_");
        }
    }

    #[must_use]
    fn get_internal_id(&self) -> String {
        self.unique_id.clone()
    }
}

impl Spell {
    /// Create a new spell with the given name and description.
    #[must_use]
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            ..Default::default()
        }
    }

    /// Set the unique_id of the spell.
    #[must_use]
    pub fn with_unique_id(mut self, unique_id: &str) -> Self {
        self.unique_id = unique_id.to_string();
        self
    }

    /// Set the lore of the spell.
    #[must_use]
    pub fn with_lore(mut self, lore: &str) -> Self {
        self.lore = lore.to_string();
        self
    }

    /// Set the talisman of the spell.
    #[must_use]
    pub fn with_talisman(mut self, talisman: SpellTalisman) -> Self {
        self.talisman = talisman;
        self
    }

    /// Set the skill of the spell.
    #[must_use]
    pub fn with_skill(mut self, skill: Skill) -> Self {
        self.skill = skill;
        self
    }

    /// Set the cast slot of the spell.
    #[must_use]
    pub fn with_cast_slot(mut self, cast_slot: CastSlot) -> Self {
        self.cast_slot = cast_slot;
        self
    }

    /// Set the cooldown of the spell.
    #[must_use]
    pub fn with_cooldown(mut self, cooldown: f32) -> Self {
        self.cooldown = cooldown;
        self
    }

    /// Set the cast time of the spell.
    #[must_use]
    pub fn with_cast_time(mut self, cast_time: f32) -> Self {
        self.cast_time = cast_time;
        self
    }

    /// Set the mana cost of the spell.
    #[must_use]
    pub fn with_mana_cost(mut self, mana_cost: f32) -> Self {
        self.mana_cost = mana_cost;
        self
    }

    /// Set the range of the spell.
    #[must_use]
    pub fn with_range(mut self, range: f32) -> Self {
        self.range = range;
        self
    }

    /// Set the speed of the spell.
    #[must_use]
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    /// Set the acceleration of the spell.
    #[must_use]
    pub fn with_acceleration(mut self, acceleration: f32) -> Self {
        self.acceleration = acceleration;
        self
    }

    /// Set the duration of the spell.
    #[must_use]
    pub fn with_duration(mut self, duration: f32) -> Self {
        self.duration = duration;
        self
    }

    /// Set the base damage of the spell.
    #[must_use]
    pub fn with_base_damage(mut self, base_damage: f32) -> Self {
        self.base_damage = base_damage;
        self
    }

    /// Set the base healing of the spell.
    #[must_use]
    pub fn with_base_healing(mut self, base_healing: f32) -> Self {
        self.base_healing = base_healing;
        self
    }

    /// Set the buffs of the spell.
    #[must_use]
    pub fn with_buffs(mut self, buffs: Vec<StatEffect>) -> Self {
        self.buffs = buffs;
        self
    }

    /// Set the debuffs of the spell.
    #[must_use]
    pub fn with_debuffs(mut self, debuffs: Vec<StatEffect>) -> Self {
        self.debuffs = debuffs;
        self
    }

    /// Set the radius of the spell.
    #[must_use]
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    /// Set the angle of the spell.
    #[must_use]
    pub fn with_angle(mut self, angle: f32) -> Self {
        self.angle = angle;
        self
    }

    /// Set the icon tileset and index of the spell.
    #[must_use]
    pub fn with_icon_tileset_index(mut self, tileset: &str, index: usize) -> Self {
        self.icon_tileset = tileset.to_string();
        self.icon_index = index;
        self
    }

    /// Set the sprite tileset and index of the spell.
    #[must_use]
    pub fn with_sprite_tileset_index(mut self, tileset: &str, index: usize) -> Self {
        self.sprite_tileset = tileset.to_string();
        self.sprite_index = index;
        self
    }

    /// Set the particles of the spell.
    #[must_use]
    pub fn with_particles(mut self, particles: Vec<SpellParticles>) -> Self {
        self.particles = particles;
        self
    }

    /// Get the unique_id of the spell.
    #[must_use]
    pub fn unique_id(&self) -> &str {
        &self.unique_id
    }

    /// Get the name of the spell.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the description of the spell.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get the lore of the spell.
    #[must_use]
    pub fn lore(&self) -> &str {
        &self.lore
    }

    /// Get the talisman of the spell.
    #[must_use]
    pub fn talisman(&self) -> &SpellTalisman {
        &self.talisman
    }

    /// Get the skill of the spell.
    #[must_use]
    pub fn skill(&self) -> Skill {
        self.skill
    }

    /// Get the cast slot of the spell.
    #[must_use]
    pub fn cast_slot(&self) -> CastSlot {
        self.cast_slot
    }

    /// Get the cooldown of the spell.
    #[must_use]
    pub fn cooldown(&self) -> f32 {
        self.cooldown
    }

    /// Get the cast time of the spell.
    #[must_use]
    pub fn cast_time(&self) -> f32 {
        self.cast_time
    }

    /// Get the mana cost of the spell.
    #[must_use]
    pub fn mana_cost(&self) -> f32 {
        self.mana_cost
    }

    /// Get the range of the spell.
    #[must_use]
    pub fn range(&self) -> f32 {
        self.range
    }

    /// Get the speed of the spell.
    #[must_use]
    pub fn speed(&self) -> f32 {
        self.speed
    }

    /// Get the acceleration of the spell.
    #[must_use]
    pub fn acceleration(&self) -> f32 {
        self.acceleration
    }

    /// Get the duration of the spell.
    #[must_use]
    pub fn duration(&self) -> f32 {
        self.duration
    }

    /// Get the base damage of the spell.
    #[must_use]
    pub fn base_damage(&self) -> f32 {
        self.base_damage
    }

    /// Get the base healing of the spell.
    #[must_use]
    pub fn base_healing(&self) -> f32 {
        self.base_healing
    }

    /// Get the buffs of the spell.
    #[must_use]
    pub fn buffs(&self) -> &[StatEffect] {
        &self.buffs
    }

    /// Get the debuffs of the spell.
    #[must_use]
    pub fn debuffs(&self) -> &[StatEffect] {
        &self.debuffs
    }

    /// Get the radius of the spell.
    #[must_use]
    pub fn radius(&self) -> f32 {
        self.radius
    }

    /// Get the angle of the spell.
    #[must_use]
    pub fn angle(&self) -> f32 {
        self.angle
    }

    /// Get the icon tileset id
    #[must_use]
    pub fn icon_tileset(&self) -> &str {
        &self.icon_tileset
    }

    /// Get the icon tileset and index of the spell.
    #[must_use]
    pub fn icon_tileset_index(&self) -> usize {
        self.icon_index
    }

    /// Get the sprite tileset id
    #[must_use]
    pub fn sprite_tileset(&self) -> &str {
        &self.sprite_tileset
    }

    /// Get the sprite tileset and index of the spell.
    #[must_use]
    pub fn sprite_tileset_index(&self) -> usize {
        self.sprite_index
    }

    /// Get the particles of the spell.
    #[must_use]
    pub fn particles(&self) -> &[SpellParticles] {
        &self.particles
    }
}

/// #### DEFAULTS FOR SERDE ####
mod spell_defaults {
    use super::SpellTalisman;
    use crate::enums::{CastSlot, CastType, Skill};

    pub(super) const fn skill() -> Skill {
        Skill::Arcanomancy
    }
    pub(super) const fn cast_slot() -> CastSlot {
        CastSlot::Cantrip
    }
    pub(super) fn talisman() -> SpellTalisman {
        SpellTalisman::default()
    }
    pub(super) const fn cast_type() -> CastType {
        CastType::Instant
    }
    pub(super) const fn mana_cost() -> f32 {
        0.0
    }
    pub(super) const fn cooldown() -> f32 {
        1.0
    }
    pub(super) const fn cast_time() -> f32 {
        0.0
    }
    pub(super) const fn range() -> f32 {
        5.0
    }
    pub(super) const fn speed() -> f32 {
        1.0
    }
    pub(super) const fn duration() -> f32 {
        5.0
    }
    pub(super) const fn acceleration() -> f32 {
        0.0
    }
    pub(super) const fn radius() -> f32 {
        0.0
    }
    pub(super) const fn angle() -> f32 {
        0.0
    }
    pub(super) const fn base_damage() -> f32 {
        0.0
    }
    pub(super) const fn base_healing() -> f32 {
        0.0
    }
}

impl<D: Hash + InternalId + 'static> TryInto<Spell> for DataFile<D> {
    type Error = ();

    fn try_into(self) -> Result<Spell, Self::Error> {
        if self.header.system != GameSystem::Spell {
            return Err(());
        }

        (&self.data as &dyn Any)
            .downcast_ref::<Spell>()
            .cloned()
            .ok_or(())
    }
}

impl<D: Hash + InternalId + 'static> TryFrom<&DataFile<D>> for Spell {
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
