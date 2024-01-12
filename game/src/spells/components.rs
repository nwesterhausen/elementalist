use bevy::prelude::*;

use crate::common::movement::MovingThingBundle;

#[derive(Bundle, Default)]
pub struct SpellBundle {
    pub moving_thing: MovingThingBundle,
    pub spell: Spell,
    pub sprite: SpriteBundle,
    pub lifetime: SpellLifetime,
}

/// The lifetime of a spell
///
/// When a spell is spawned, it is given a lifetime. When the lifetime expires,
/// the spell is despawned.
#[derive(Debug, Clone, Copy, PartialEq, Component, Default)]
pub struct SpellLifetime {
    /// The remaining lifetime of the spell in seconds
    pub remaining: f32,
    /// The maximum lifetime of the spell in seconds
    pub max: f32,
}

impl SpellLifetime {
    /// Creates a new spell lifetime with the given maximum lifetime
    pub fn new(max: f32) -> Self {
        Self {
            remaining: max,
            max,
        }
    }
    /// Updates the spell's lifetime using `time.delta_seconds()`
    pub fn update(&mut self, delta: f32) {
        self.remaining -= delta;
    }
    /// Returns true if the spell is expired
    pub fn is_expired(&self) -> bool {
        self.remaining <= 0.0
    }
}

/// Despawns spells when their lifetime expires, and updates their lifetime
pub fn despawn_expired_spells(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut SpellLifetime)>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.update(time.delta_seconds());
        if lifetime.is_expired() {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Default)]
pub enum Spell {
    #[default]
    Empty,
    Firebolt,
    StoneDart,
}

#[derive(Event)]
pub struct CastSpell(pub Spell);
