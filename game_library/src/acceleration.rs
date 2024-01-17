use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

/// Simple acceleration component containing a 2D vector
///
/// Defaults to `Vec2::ZERO`
#[derive(Component, Debug, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Acceleration {
    /// The acceleration value
    pub value: Vec2,
}

impl Default for Acceleration {
    fn default() -> Self {
        Self { value: Vec2::ZERO }
    }
}

impl Acceleration {
    /// Creates a new acceleration component with the given value
    #[must_use]
    pub fn new(value: Vec2) -> Self {
        Self { value }
    }
}
