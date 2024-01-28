//! Health component used in the game

use bevy::{prelude::*, reflect::Reflect};
use bevy_inspector_egui::prelude::*;

use crate::{progress_bar::Percentage, Attribute};

/// Health component to hold an entity's health value
#[derive(Component, Default, Debug, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Health {
    /// The health value
    pub value: Attribute,
}

impl Health {
    /// Creates a new health component with the given max value.
    ///
    /// Health will be set to the max value.
    #[must_use]
    pub fn new(value: u32) -> Self {
        Self {
            value: Attribute::new(value),
        }
    }
    /// Instantly kill the entity, setting health to 0.
    pub fn kill(&mut self) {
        self.value.set(0_u32);
    }

    /// Check if dead (passes through to `Attribute::is_empty`)
    #[must_use]
    pub const fn is_dead(&self) -> bool {
        self.value.is_empty()
    }
}

impl Percentage for Health {
    fn percentage(&self) -> f32 {
        self.value.remaining()
    }
}
