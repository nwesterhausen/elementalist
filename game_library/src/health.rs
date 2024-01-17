use bevy::{prelude::*, reflect::Reflect};
use bevy_inspector_egui::prelude::*;

use crate::Attribute;

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
    pub fn new(value: u32) -> Self {
        Self {
            value: Attribute::new(value),
        }
    }
}
