//! Configuration of a progress bar.
//!
//! This is used to configure the progress bar.

use bevy::prelude::*;

use super::{BarState, ColorScheme, Percentage};

/// Configuration of a progress bar.
#[derive(Component, Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct ProgressBarConfig<T: Percentage + Component> {
    /// The color scheme to use for the progress bar.
    pub color_scheme: ColorScheme,
    /// The size of the progress bar.
    ///
    /// This is the size to draw the background for the bar, and the foreground bar will be drawn inside of this. At
    /// full, the foreground bar will be the same size as the background bar.
    pub size: Vec2,
    /// The relative position of the progress bar to the entity it will be attached to.
    ///
    /// This is relative to the [`Transform.translation`] of the entity.
    pub position_translation: Vec3,

    _marker: std::marker::PhantomData<T>,
}

impl<T: Percentage + Component> Default for ProgressBarConfig<T> {
    fn default() -> Self {
        Self {
            color_scheme: ColorScheme::default(),
            size: Vec2::new(100.0, 10.0),
            position_translation: Vec3::new(0.0, 0.0, 0.0),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Percentage + Component> ProgressBarConfig<T> {
    /// Create a new progress bar config with the given color scheme.
    #[must_use]
    pub fn new(color_scheme: ColorScheme) -> Self {
        Self {
            color_scheme,
            ..Default::default()
        }
    }

    /// Update the progress bar config with the given color scheme.
    #[must_use]
    pub const fn with_color_scheme(mut self, color_scheme: ColorScheme) -> Self {
        self.color_scheme = color_scheme;
        self
    }
    /// Update the progress bar's color scheme with the given background color.
    #[must_use]
    pub const fn with_background_color(mut self, color: Color) -> Self {
        self.color_scheme.background = color;
        self
    }
    /// Update the progress bar's color scheme with the given color for the given state.
    #[must_use]
    pub fn with_color(mut self, state: &BarState, color: Color) -> Self {
        self.color_scheme.set_color(state, color);
        self
    }
    /// Update the progress bar's color scheme with the given moderate cutoff.
    #[must_use]
    pub fn with_moderate_cutoff(mut self, cutoff: f32) -> Self {
        self.color_scheme.set_moderate_cutoff(cutoff);
        self
    }
    /// Update the progress bar's color scheme with the given critical cutoff.
    #[must_use]
    pub fn with_critical_cutoff(mut self, cutoff: f32) -> Self {
        self.color_scheme.set_critical_cutoff(cutoff);
        self
    }
    /// Update the progress bar to use a single color for all states.
    #[must_use]
    pub fn with_single_color(mut self, color: Color) -> Self {
        self.color_scheme.set_single_color(color);
        self
    }
    /// Update the progress bar's size.
    #[must_use]
    pub const fn with_size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }
    /// Update the progress bar's relative position.
    #[must_use]
    pub const fn with_position_translation(mut self, position_translation: Vec3) -> Self {
        self.position_translation = position_translation;
        self
    }

    /// Set the size of the progress bar.
    pub fn set_size(&mut self, size: Vec2) {
        self.size = size;
    }
    /// Set the relative position of the progress bar.
    pub fn set_position_translation(&mut self, position_translation: Vec3) {
        self.position_translation = position_translation;
    }
    /// Set the color scheme of the progress bar.
    pub fn set_color_scheme(&mut self, color_scheme: ColorScheme) {
        self.color_scheme = color_scheme;
    }

    /// Get the background color.
    #[must_use]
    pub const fn background_color(&self) -> Color {
        self.color_scheme.background
    }
    /// Get the foreground color for the given T.
    #[must_use]
    pub fn color(&self, percentage: &T) -> Color {
        self.color_scheme.get_color(percentage)
    }
    /// Get the realized [`Transform`] for the progress bar background.
    ///
    /// This adjusts the translation of the background to be relative to the entity's translation.
    #[must_use]
    pub fn background_transform(&self, entity_transform: &Transform) -> Transform {
        let mut transform = *entity_transform;
        transform.translation += self.position_translation;
        transform
    }
    /// Get the realized [`Transform`] for the progress bar foreground.
    ///
    /// This adjusts the translation of the foreground to be relative to the entity's translation. It also adjusts the
    /// translation of the foreground to be centered on the entity's translation.
    #[must_use]
    pub fn foreground_transform(&self, entity_transform: &Transform, percentage: &T) -> Transform {
        let mut transform = *entity_transform;
        transform.translation += self.position_translation;
        transform.translation.x += self.size.x * percentage.percentage() / 2.0;
        transform
    }
    /// Get the mesh for the progress bar background.
    ///
    /// This is a [`Quad`] with the size of the progress bar.
    #[must_use]
    pub fn background_mesh(&self) -> Mesh {
        shape::Quad::new(self.size).into()
    }
    /// Get the mesh for the progress bar foreground.
    ///
    /// This is a [`Quad`] with the size of the progress bar foreground, which is the size of the progress
    /// bar background scaled by the percentage.
    #[must_use]
    pub fn foreground_mesh(&self, percentage: &T) -> Mesh {
        shape::Quad::new(Vec2::new(
            self.size.x * percentage.percentage(),
            self.size.y,
        ))
        .into()
    }
    /// Get the current [`BarState`] for the given percentage.
    #[must_use]
    pub fn get_state(&self, percentage: &T) -> BarState {
        self.color_scheme
            .cutoffs()
            .get_state(percentage.percentage())
    }
    /// Get the color for the given [`BarState`].
    #[must_use]
    pub const fn color_for_state(&self, state: &BarState) -> Color {
        self.color_scheme.bar.get_state(state)
    }
}
