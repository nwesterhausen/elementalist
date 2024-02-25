//! Spell lifetime component (or how long a spell will last)

use bevy::prelude::*;

/// The lifetime of a spell
///
/// When a spell is spawned, it is given a lifetime. When the lifetime expires,
/// the spell is despawned.
#[derive(Debug, Clone, Copy, PartialEq, Component, Default, Reflect)]
pub struct SpellLifetime {
    /// The remaining lifetime of the spell in seconds
    remaining: f32,
    /// The maximum lifetime of the spell in seconds
    max: f32,
}

impl SpellLifetime {
    /// Creates a new spell lifetime with the given maximum lifetime
    #[must_use]
    pub const fn new(max: f32) -> Self {
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
    #[must_use]
    pub fn is_expired(&self) -> bool {
        self.remaining <= 0.0
    }

    /// Returns the remaining lifetime of the spell
    #[must_use]
    pub fn remaining(&self) -> f32 {
        self.remaining
    }
    /// Returns the maximum lifetime of the spell
    #[must_use]
    pub fn max(&self) -> f32 {
        self.max
    }
}
