use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// A volume setting, which should be an integer between 0 and 100.
///
/// Can be called with .into() to convert from any integer type, or from a float.
///
/// Can be converted into a `u32` (representing the volume as a percentage) or a `f32` (representing
/// the volume as a decimal; i.e. 50% volume would be 0.5 as `f32`).
///
/// Keeps tract of whether or not the volume is muted.
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Volume {
    /// The volume as a percentage
    pub value: u32,
    /// Whether or not the volume is muted
    pub muted: bool,
}

impl Volume {
    /// The minimum volume is 0%
    pub const MIN: u32 = 0;
    /// The maximum volume is 100%
    pub const MAX: u32 = 100;

    /// Sets the volume to the given value, clamping it between 0 and 100.
    ///
    /// ## Arguments
    ///
    /// * `value` - The value to set the volume to. This can be a `u32`, `u16`, `u8`, `i32`, `i16`,
    /// `i8`, `f32`, or `f64`.
    ///
    pub fn set(&mut self, value: impl Into<Self>) {
        let new_volume = value.into();
        self.value = new_volume.value;
    }

    /// Returns the volume as a percentage.
    ///
    /// For example, 50% volume would be 50.
    pub fn as_percentage(&self) -> u32 {
        self.value
    }

    /// Returns the volume as a decimal.
    ///
    /// For example, 50% volume would be 0.5.
    pub fn as_decimal(&self) -> f32 {
        self.value as f32 / 100.0
    }

    /// Returns whether or not the volume is muted.
    pub fn is_muted(&self) -> bool {
        self.muted
    }

    /// Mutes the volume
    pub fn mute(&mut self) {
        self.muted = true;
    }

    /// Unmutes the volume
    pub fn unmute(&mut self) {
        self.muted = false;
    }

    /// Gets the effective volume, taking into account whether or not the volume is muted.
    ///
    /// If the volume is muted, this will return 0. Otherwise, it will return the volume as a
    /// percentage.
    pub fn effective_volume(&self) -> u32 {
        if self.muted {
            0
        } else {
            self.value
        }
    }

    /// Gets the effective volume as a decimal, taking into account whether or not the volume is
    ///
    /// If the volume is muted, this will return 0.0. Otherwise, it will return the volume as a
    /// decimal.
    pub fn effective_volume_as_decimal(&self) -> f32 {
        if self.muted {
            0.0
        } else {
            self.value as f32 / 100.0
        }
    }
}

impl std::default::Default for Volume {
    fn default() -> Self {
        Self {
            value: 100,
            muted: false,
        }
    }
}

impl From<u32> for Volume {
    fn from(value: u32) -> Self {
        let clamped_value = value.min(Self::MAX).max(Self::MIN);
        Self {
            value: clamped_value,
            muted: false,
        }
    }
}

impl From<u16> for Volume {
    fn from(value: u16) -> Self {
        let clamped_value = (value as u32).min(Self::MAX).max(Self::MIN);
        Self {
            value: clamped_value,
            muted: false,
        }
    }
}

impl From<u8> for Volume {
    fn from(value: u8) -> Self {
        let clamped_value = (value as u32).min(Self::MAX).max(Self::MIN);
        Self {
            value: clamped_value,
            muted: false,
        }
    }
}

impl From<i32> for Volume {
    fn from(value: i32) -> Self {
        let clamped_value = (value as u32).min(Self::MAX).max(Self::MIN);
        Self {
            value: clamped_value,
            muted: false,
        }
    }
}

impl From<i16> for Volume {
    fn from(value: i16) -> Self {
        let clamped_value = (value as u32).min(Self::MAX).max(Self::MIN);
        Self {
            value: clamped_value,
            muted: false,
        }
    }
}

impl From<i8> for Volume {
    fn from(value: i8) -> Self {
        let clamped_value = (value as u32).min(Self::MAX).max(Self::MIN);
        Self {
            value: clamped_value,
            muted: false,
        }
    }
}

impl From<f32> for Volume {
    fn from(value: f32) -> Self {
        let clamped_value = ((value * 100.0) as u32).min(Self::MAX).max(Self::MIN);
        Self {
            value: clamped_value,
            muted: false,
        }
    }
}

impl From<f64> for Volume {
    fn from(value: f64) -> Self {
        let clamped_value = ((value * 100.0) as u32).min(Self::MAX).max(Self::MIN);
        Self {
            value: clamped_value,
            muted: false,
        }
    }
}

impl From<Volume> for u32 {
    fn from(value: Volume) -> Self {
        value.value
    }
}

impl From<Volume> for f32 {
    fn from(value: Volume) -> Self {
        value.value as f32 / 100.0
    }
}
