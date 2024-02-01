//! Colors for a progress bar.

use bevy::prelude::*;

use super::{BarState, Percentage};

/// The colors used in a progress bar.
#[derive(Debug, Clone)]
pub struct ColorScheme {
    /// The background color of the bar. Defaults to [`Color::NONE`].
    pub background: Color,
    /// The color of the bar, for three different states: Ok, Moderate, and Critical.
    pub bar: BarColors,
    /// The cutoffs for the Moderate and Critical states.
    ///
    /// These are percentages, so they should be between 0.0 and 1.0, and the Moderate cutoff
    /// should be less than the Critical cutoff.
    ///
    /// The default values are 0.55 and 0.2, respectively.
    pub cutoffs: BarStateCutoffs,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            background: Color::NONE,
            bar: BarColors::default(),
            cutoffs: BarStateCutoffs::default(),
        }
    }
}

impl ColorScheme {
    /// Get the color needed for the given percentage.
    #[must_use]
    pub fn get_color_direct(&self, percentage: f32) -> Color {
        let state = self.cutoffs.get_state(percentage);
        self.bar.get_state(&state)
    }
    /// Get the color needed for the given object that implements [`Percentage`].
    #[must_use]
    pub fn get_color<T: Percentage>(&self, percentage: &T) -> Color {
        self.get_color_direct(percentage.percentage())
    }
    /// Set the background color.
    pub fn set_background(&mut self, color: Color) {
        self.background = color;
    }
    /// Set the color for the given state.
    pub fn set_color(&mut self, state: &BarState, color: Color) {
        match state {
            BarState::Ok => self.bar.ok = color,
            BarState::Moderate => self.bar.moderate = color,
            BarState::Critical => self.bar.critical = color,
        }
    }
    /// Set the moderate cutoff.
    pub fn set_moderate_cutoff(&mut self, cutoff: f32) {
        self.cutoffs.moderate = cutoff;
    }
    /// Set the critical cutoff.
    pub fn set_critical_cutoff(&mut self, cutoff: f32) {
        self.cutoffs.critical = cutoff;
    }
    /// Set the same color for all states.
    pub fn set_single_color(&mut self, color: Color) {
        self.bar.ok = color;
        self.bar.moderate = color;
        self.bar.critical = color;
    }
    /// Get the bar cutoffs.
    #[must_use]
    pub const fn cutoffs(&self) -> &BarStateCutoffs {
        &self.cutoffs
    }
}

/// The colors used to draw the progress bar.
#[derive(Debug, Clone)]
pub struct BarColors {
    /// The color of the bar when the value is in the Ok state.
    pub ok: Color,
    /// The color of the bar when the value is in the Moderate state.
    pub moderate: Color,
    /// The color of the bar when the value is in the Critical state.
    pub critical: Color,
}

impl Default for BarColors {
    fn default() -> Self {
        Self {
            ok: Color::GREEN,
            moderate: Color::YELLOW,
            critical: Color::RED,
        }
    }
}

impl BarColors {
    /// Get the color for the given state.
    pub const fn get_state(&self, state: &BarState) -> Color {
        match state {
            BarState::Ok => self.ok,
            BarState::Moderate => self.moderate,
            BarState::Critical => self.critical,
        }
    }
}

/// The cutoffs for the Moderate and Critical states.
#[derive(Debug, Clone)]
pub struct BarStateCutoffs {
    /// The cutoff for the Moderate state.
    pub moderate: f32,
    /// The cutoff for the Critical state.
    pub critical: f32,
}

impl Default for BarStateCutoffs {
    fn default() -> Self {
        Self {
            moderate: 0.55,
            critical: 0.2,
        }
    }
}

impl BarStateCutoffs {
    /// Get the state for the given percentage.
    pub fn get_state(&self, percentage: f32) -> BarState {
        if percentage > self.moderate {
            BarState::Ok
        } else if percentage > self.critical {
            BarState::Moderate
        } else {
            BarState::Critical
        }
    }
}
