use bevy::ecs::{component::Component, system::Resource};
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
    /// * `value` - The value to set the volume to.
    ///
    /// ## Note
    ///
    /// This converts the value into a `u32` before clamping it.
    pub fn set(&mut self, value: impl Into<u32>) {
        self.value = value.into().min(Self::MAX).max(Self::MIN);
    }

    /// Returns the volume, regardless of whether or not the volume is muted.
    pub fn raw_volume<T>(&self) -> T
    where
        T: std::convert::From<u32>,
    {
        std::convert::From::from(self.value)
    }

    /// Returns the volume or 0 if the volume is muted.
    pub fn volume<T>(&self) -> T
    where
        T: std::convert::From<u32>,
    {
        if self.muted {
            std::convert::From::from(0)
        } else {
            std::convert::From::from(self.value)
        }
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
}

impl std::default::Default for Volume {
    fn default() -> Self {
        Self {
            value: 50,
            muted: false,
        }
    }
}

impl std::fmt::Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.muted {
            write!(f, "muted")
        } else {
            write!(f, "{}%", self.value)
        }
    }
}

impl From<u64> for Volume {
    fn from(value: u64) -> Self {
        let clamped_value = (value as u32).min(Self::MAX).max(Self::MIN);
        Self {
            value: clamped_value,
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

impl From<i64> for Volume {
    fn from(value: i64) -> Self {
        let clamped_value = value.min(Self::MAX as i64).max(Self::MIN as i64);
        Self {
            value: clamped_value as u32,
            muted: false,
        }
    }
}

impl From<i32> for Volume {
    fn from(value: i32) -> Self {
        let clamped_value = value.min(Self::MAX as i32).max(Self::MIN as i32);
        Self {
            value: clamped_value as u32,
            muted: false,
        }
    }
}

impl From<i16> for Volume {
    fn from(value: i16) -> Self {
        let clamped_value = value.min(Self::MAX as i16).max(Self::MIN as i16);
        Self {
            value: clamped_value as u32,
            muted: false,
        }
    }
}

impl From<i8> for Volume {
    fn from(value: i8) -> Self {
        let clamped_value = value.min(Self::MAX as i8).max(Self::MIN as i8);
        Self {
            value: clamped_value as u32,
            muted: false,
        }
    }
}

impl From<f32> for Volume {
    fn from(value: f32) -> Self {
        let clamped_value = ((value * 100.0) as i32)
            .min(Self::MAX as i32)
            .max(Self::MIN as i32);
        Self {
            value: clamped_value as u32,
            muted: false,
        }
    }
}

impl From<f64> for Volume {
    fn from(value: f64) -> Self {
        let clamped_value = ((value * 100.0) as i32)
            .min(Self::MAX as i32)
            .max(Self::MIN as i32);
        Self {
            value: clamped_value as u32,
            muted: false,
        }
    }
}
