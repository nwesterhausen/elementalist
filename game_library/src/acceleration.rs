//! Acceleration component
//!
//! Contains the acceleration component and its implementations. The acceleration component is part of
//! the [`elementalist_game_library::MovementBundle`]. It is used to store the acceleration value of an entity.
//! (Acceleration is the change in velocity over time).

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
    ///
    /// # Arguments
    ///
    /// * `value` - The acceleration value
    ///
    /// # Returns
    ///
    /// * Self - The new acceleration component
    ///
    /// # Examples
    ///
    /// ```
    /// use elementalist_game_library::Acceleration;
    /// use bevy::math::Vec2;
    ///
    /// let acceleration = Acceleration::new(Vec2::new(1.0, 1.0));
    /// ```
    #[must_use]
    pub const fn new(value: Vec2) -> Self {
        Self { value }
    }
}
