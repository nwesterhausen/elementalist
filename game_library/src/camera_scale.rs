//! Camera scale level component.
//!
//! This component stores the current camera scale level. This is used to zoom the camera in and out.
//!
//! Typical usage would be to add this component to the main camera entity, and then use the
//! [`CameraScaleLevel::zoom_in`] and [`CameraScaleLevel::zoom_out`] methods to change the scale
//! level.
//!
//! # Default Value
//!
//! The default value is `0.3`. This is the default zoom level for the game.
//!
//! # Possible Values
//!
//! The possible values are `0.1`, `0.25`, `0.3`, `0.5`, and `1.0`.
//!
//! These are available as constants on the struct: [`CameraScaleLevel::LEVELS`].
//!
//! # Example
//!
//! ```no_run
//! use bevy::prelude::*;
//! use elementalist_game_library::CameraScaleLevel;
//!
//! #[derive(Component)]
//! struct MainCamera;
//!
//! /// Spawns a basic camera
//! pub fn setup_camera(mut commands: Commands) {
//!   commands.spawn((
//!       Camera2dBundle::default(),
//!       MainCamera,
//!       CameraScaleLevel::default(),
//!   ));
//! }
//!
//! /// A system to run on [Update] which zooms the camera based on the current scale level
//! pub fn zoom_camera(
//!   mut query: Query<(&mut OrthographicProjection, &CameraScaleLevel), With<MainCamera>>,
//! ) {
//!   for (mut projection, scale_level) in &mut query {
//!       projection.scale = scale_level.value();
//!   }
//! }
//!
//! /// A system to run on [KeyboardInput] which zooms the camera in or out
//! pub fn zoom_camera_system(
//!  mut query: Query<&mut CameraScaleLevel, With<MainCamera>>,
//!  keyboard_input: Res<Input<KeyCode>>,
//! ) {
//!  for mut scale_level in &mut query {
//!     if keyboard_input.just_pressed(KeyCode::Equals) {
//!        scale_level.zoom_out();
//!    }
//!   if keyboard_input.just_pressed(KeyCode::Minus) {
//!       scale_level.zoom_in();
//!    }
//!   }
//! }
//! ```

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use serde::{Deserialize, Serialize};

/// Component that stores the current camera scale level.
///
/// This is used to zoom the camera in and out.
///
/// # Default Value
///
/// The default value is `0.3`.
///
/// # Possible Values
///
/// The possible values are `0.1`, `0.25`, `0.3`, `0.5`, and `1.0`.
///
/// These are available as constants on the struct:
///
/// * [`CameraScaleLevel::LEVELS`]
#[derive(Component, Reflect, InspectorOptions, Clone, Copy, Debug, Serialize, Deserialize)]
#[reflect(Component, InspectorOptions)]
#[allow(clippy::module_name_repetitions)]
pub struct CameraScaleLevel {
    value: f32,
    index: usize,
}

impl Default for CameraScaleLevel {
    fn default() -> Self {
        Self {
            value: 0.3,
            index: 2,
        }
    }
}

impl CameraScaleLevel {
    /// The possible values for the camera scale level.
    pub const LEVELS: [f32; 5] = [0.1, 0.25, 0.3, 0.5, 1.0];

    /// Returns the current camera scale level.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use elementalist_game_library::CameraScaleLevel;
    ///
    /// let level = CameraScaleLevel::default();
    ///
    /// assert_eq!(level.value(), 0.3);
    /// ```
    #[must_use]
    pub const fn value(&self) -> f32 {
        self.value
    }

    /// Increases the camera scale level. If the level is already at the maximum, it will not
    /// increase. Note that increasing the level (zooming in) means the scale value will be smaller.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use elementalist_game_library::CameraScaleLevel;
    ///
    /// let mut level = CameraScaleLevel::default();
    ///
    /// assert_eq!(level.value(), 0.3);
    ///
    /// level.zoom_out();
    ///
    /// assert_eq!(level.value(), 0.5);
    /// ```
    pub fn zoom_out(&mut self) {
        self.index += 1;

        if self.index >= Self::LEVELS.len() {
            self.index = Self::LEVELS.len() - 1;
        }

        self.value = Self::LEVELS[self.index];
    }

    /// Decreases the camera scale level. If the level is already at the minimum, it will not
    /// decrease. Note that decreasing the level (zooming out) means the scale value will be larger.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use elementalist_game_library::CameraScaleLevel;
    ///
    /// let mut level = CameraScaleLevel::default();
    ///
    /// assert_eq!(level.value(), 0.3);
    ///
    /// level.zoom_in();
    ///
    /// assert_eq!(level.value(), 0.25);
    /// ```
    pub fn zoom_in(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }

        self.value = Self::LEVELS[self.index];
    }
}

impl From<f32> for CameraScaleLevel {
    fn from(value: f32) -> Self {
        let mut index = 3;
        if let Some(found_index) = Self::LEVELS
            .iter()
            .position(|&v| (v - value).abs() < f32::EPSILON)
        {
            index = found_index;
        } else {
            tracing::warn!("invalid camera scale level `{}`, using default", value);
        }
        Self { value, index }
    }
}

impl From<CameraScaleLevel> for f32 {
    fn from(level: CameraScaleLevel) -> Self {
        level.value
    }
}

impl std::fmt::Display for CameraScaleLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}", self.value)
    }
}
