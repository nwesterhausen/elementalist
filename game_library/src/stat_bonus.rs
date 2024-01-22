//! Stat bonuses are values that can be increased or decreased. These are not used directly but
//! are multiplied against a stat's base value to get the final value.

use bevy::{
    ecs::{component::Component, system::Resource},
    reflect::Reflect,
};
use bevy_inspector_egui::inspector_options::{InspectorOptions, ReflectInspectorOptions};
use serde::{Deserialize, Serialize};

/// A stat bonus is a value that can be increased or decreased. These should be considered
/// values that operate on a percentage scale, where 1.0 is 100% (i.e. no change), 0.5 is 50%
/// (i.e. half), and 2.0 is 200% (i.e. double).
///
/// This is a very simple struct, but it's useful for things like damage multipliers, or
/// movement speed multipliers.
///
/// # Note
///
/// This does not support negative values, they don't make sense for a percentage based
/// multiplier. If a value would become negative, it will be clamped to 0.0 instead.
#[derive(
    Debug,
    Resource,
    Clone,
    Copy,
    PartialEq,
    Reflect,
    Component,
    Serialize,
    Deserialize,
    InspectorOptions,
)]
#[reflect(InspectorOptions)]
#[serde(rename_all = "camelCase")]
pub struct StatBonus {
    /// The percentage bonus to apply to the stat.
    #[inspector(min = 0.0, speed = 0.1)]
    pub value: f32,
}

impl StatBonus {
    /// Creates a new stat bonus with the given value.
    #[must_use]
    pub const fn new(value: f32) -> Self {
        Self { value }
    }

    /// Returns the value of the stat bonus.
    #[must_use]
    pub const fn value(&self) -> f32 {
        self.value
    }

    /// Sets the value of the stat bonus.
    pub fn set_value(&mut self, value: f32) {
        self.value = value;
        // Clamp
        if self.value < 0.0 {
            self.value = 0.0;
        }
    }

    /// Adds the given value to the stat bonus.
    pub fn add_value(&mut self, value: f32) {
        self.value += value;
        // Clamp
        if self.value < 0.0 {
            self.value = 0.0;
        }
    }

    /// Subtracts the given value from the stat bonus.
    pub fn subtract_value(&mut self, value: f32) {
        self.value -= value;
        // Clamp
        if self.value < 0.0 {
            self.value = 0.0;
        }
    }

    /// Multiplies the stat bonus by the given value.
    pub fn multiply_value(&mut self, value: f32) {
        self.value *= value;
        // Clamp
        if self.value < 0.0 {
            self.value = 0.0;
        }
    }

    /// Divides the stat bonus by the given value.
    pub fn divide_value(&mut self, value: f32) {
        // Guard against divide by zero
        if value == 0.0 {
            self.value = 0.0;
            return;
        }
        self.value /= value;
        // Clamp
        if self.value < 0.0 {
            self.value = 0.0;
        }
    }

    /// Percentage-based addition, where the given value is a percentage to add to the stat
    /// bonus. For example, if the stat bonus is 1.0, and the given value is 50, the result
    /// will be 1.5.
    ///
    /// This is to provide a more convenient way to add percentages to the stat bonus.
    pub fn add_percent(&mut self, percent: impl Into<f32>) {
        let percentage = percent.into() / 100.0;
        self.add_value(percentage);
    }

    /// Percentage-based subtraction, where the given value is a percentage to subtract from
    /// the stat bonus. For example, if the stat bonus is 1.0, and the given value is 50, the
    /// result will be 0.5.
    ///
    /// This is to provide a more convenient way to subtract percentages from the stat bonus.
    pub fn subtract_percent(&mut self, percent: impl Into<f32>) {
        let percentage = percent.into() / 100.0;
        self.subtract_value(percentage);
    }

    /// Percentage-based multiplication, where the given value is a percentage to multiply
    /// the stat bonus by. For example, if the stat bonus is 1.0, and the given value is 75,
    /// the result will be 0.75.
    ///
    /// This is to provide a more convenient way to multiply percentages to the stat bonus.
    pub fn multiply_percent(&mut self, percent: impl Into<f32>) {
        let percentage = percent.into() / 100.0;
        self.multiply_value(percentage);
    }

    /// Percentage-based division, where the given value is a percentage to divide the stat
    /// bonus by. For example, if the stat bonus is 1.0, and the given value is 50, the
    /// result will be 2.0.
    ///
    /// This is to provide a more convenient way to divide percentages from the stat bonus.
    pub fn divide_percent(&mut self, percent: impl Into<f32>) {
        let percentage = percent.into() / 100.0;
        self.divide_value(percentage);
    }
}

impl std::default::Default for StatBonus {
    fn default() -> Self {
        Self { value: 1.0 }
    }
}

impl std::ops::Add for StatBonus {
    type Output = f32;

    fn add(self, rhs: Self) -> Self::Output {
        let result = self.value + rhs.value;
        if result < 0.0 {
            0.0
        } else {
            result
        }
    }
}

impl std::ops::AddAssign for StatBonus {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
        if self.value < 0.0 {
            self.value = 0.0;
        }
    }
}

impl std::ops::Sub for StatBonus {
    type Output = f32;

    fn sub(self, rhs: Self) -> Self::Output {
        let result = self.value - rhs.value;
        if result < 0.0 {
            0.0
        } else {
            result
        }
    }
}

impl std::ops::SubAssign for StatBonus {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
        if self.value < 0.0 {
            self.value = 0.0;
        }
    }
}

impl std::ops::Mul for StatBonus {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        let result = self.value * rhs.value;
        if result < 0.0 {
            0.0
        } else {
            result
        }
    }
}

impl std::ops::MulAssign for StatBonus {
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= rhs.value;
        if self.value < 0.0 {
            self.value = 0.0;
        }
    }
}

impl std::ops::Div for StatBonus {
    type Output = f32;

    fn div(self, rhs: Self) -> Self::Output {
        let result = self.value / rhs.value;
        if result < 0.0 {
            0.0
        } else {
            result
        }
    }
}

impl std::ops::DivAssign for StatBonus {
    fn div_assign(&mut self, rhs: Self) {
        self.value /= rhs.value;
        if self.value < 0.0 {
            self.value = 0.0;
        }
    }
}

impl std::ops::Neg for StatBonus {
    type Output = f32;

    fn neg(self) -> Self::Output {
        -self.value
    }
}

impl std::cmp::PartialOrd for StatBonus {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for StatBonus {
    #[doc = "We are comparing floats, but we aren't so worried about precision because these are just"]
    #[doc = "percentages. So we multiply by 100, convert to u32, and compare those."]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_value = self.value * 100.0;
        let other_value = other.value * 100.0;

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        (self_value as u32).cmp(&(other_value as u32))
    }
}

impl std::fmt::Display for StatBonus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:+.2}%", self.value * 100.0)
    }
}

impl std::cmp::Eq for StatBonus {}
