use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

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
#[derive(Component, Reflect, InspectorOptions, Clone, Copy)]
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
    /// ```
    /// use game_library::CameraScaleLevel;
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
    /// ```
    /// use game_library::CameraScaleLevel;
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
    /// ```
    /// use game_library::CameraScaleLevel;
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
