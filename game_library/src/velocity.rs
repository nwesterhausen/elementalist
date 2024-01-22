use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

/// Simple velocity component containing a 2D vector
///
/// Defaults to `Vec2::ZERO`
#[derive(Component, Debug, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Velocity {
    /// The velocity value
    pub value: Vec2,
}

impl Default for Velocity {
    fn default() -> Self {
        Self { value: Vec2::ZERO }
    }
}

impl Velocity {
    /// Creates a new velocity component with the given value
    #[must_use]
    pub const fn new(value: Vec2) -> Self {
        Self { value }
    }
}
