//! Mana component used in the game.

use bevy::{prelude::*, reflect::Reflect};
use bevy_health_bar3d::prelude::Percentage;
use bevy_inspector_egui::prelude::*;

use crate::Attribute;

/// Mana component to hold an entity's mana value
#[derive(Component, Default, Debug, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Mana {
    /// The mana value
    pub value: Attribute,
}

impl Mana {
    /// Creates a new mana component with the given max value.
    #[must_use]
    pub fn new(value: u32) -> Self {
        Self {
            value: Attribute::new(value),
        }
    }
}

impl Percentage for Mana {
    fn value(&self) -> f32 {
        self.value.remaining()
    }
}
