use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::Volume;

/// A bundle containing all audio settings, so that it can easily be passed around
/// as a resource.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Resource, Component, Serialize, Deserialize)]
pub struct AudioSettingsBundle {
    pub game_volume: Volume,
    pub music_volume: Volume,
    pub sound_effects_volume: Volume,
    pub music_enabled: bool,
}

impl std::default::Default for AudioSettingsBundle {
    fn default() -> Self {
        Self {
            music_enabled: true,
            game_volume: Volume::default(),
            music_volume: Volume::default(),
            sound_effects_volume: Volume::default(),
        }
    }
}

impl AudioSettingsBundle {
    /// Sets the game volume to the given value, clamping it between 0 and 100.
    ///
    /// ## Arguments
    ///
    /// * `value` - The value to set the volume to. This can be a `u32`, `u16`, `u8`, `i32`, `i16`,
    /// `i8`, `f32`, or `f64`.
    ///
    pub fn set_game_volume(&mut self, value: impl Into<Volume>) {
        self.game_volume.set(value);
    }

    /// Sets the music volume to the given value, clamping it between 0 and 100.
    ///
    /// ## Arguments
    ///
    /// * `value` - The value to set the volume to. This can be a `u32`, `u16`, `u8`, `i32`, `i16`,
    /// `i8`, `f32`, or `f64`.
    ///
    pub fn set_music_volume(&mut self, value: impl Into<Volume>) {
        self.music_volume.set(value);
    }

    /// Sets the sound effects volume to the given value, clamping it between 0 and 100.
    ///
    /// ## Arguments
    ///
    /// * `value` - The value to set the volume to. This can be a `u32`, `u16`, `u8`, `i32`, `i16`,
    /// `i8`, `f32`, or `f64`.
    ///
    pub fn set_sound_effects_volume(&mut self, value: impl Into<Volume>) {
        self.sound_effects_volume.set(value);
    }

    /// Returns the game volume as a percentage, taking into account whether or not it is muted.
    ///
    /// For example, 50% volume would be 50. If the volume is muted, this will return 0.
    pub fn game_volume_as_percentage(&self) -> u32 {
        self.game_volume.effective_volume()
    }

    /// Returns the music volume as a percentage, taking into account whether or not it is muted.
    ///
    /// For example, 50% volume would be 50. If the volume is muted, this will return 0.
    pub fn music_volume_as_percentage(&self) -> u32 {
        self.music_volume.effective_volume()
    }

    /// Returns the sound effects volume as a percentage, taking into account whether or not it is
    /// muted.
    ///
    /// For example, 50% volume would be 50. If the volume is muted, this will return 0.
    pub fn sound_effects_volume_as_percentage(&self) -> u32 {
        self.sound_effects_volume.effective_volume()
    }

    /// Returns the game volume as a decimal, taking into account whether or not it is muted.
    ///
    /// For example, 50% volume would be 0.5. If the volume is muted, this will return 0.0
    pub fn game_volume_as_decimal(&self) -> f32 {
        self.game_volume.effective_volume_as_decimal()
    }

    /// Returns the music volume as a decimal, taking into account whether or not it is muted.
    ///
    /// For example, 50% volume would be 0.5. If the volume is muted, this will return 0.0
    pub fn music_volume_as_decimal(&self) -> f32 {
        self.music_volume.effective_volume_as_decimal()
    }

    /// Returns the sound effects volume as a decimal, taking into account whether or not it is
    /// muted.
    ///
    /// For example, 50% volume would be 0.5. If the volume is muted, this will return 0.0
    pub fn sound_effects_volume_as_decimal(&self) -> f32 {
        self.sound_effects_volume.effective_volume_as_decimal()
    }

    /// Returns whether or not the game volume is muted.
    ///
    /// This is equivalent to calling `self.game_volume.is_muted()`.
    pub fn is_game_volume_muted(&self) -> bool {
        self.game_volume.is_muted()
    }

    /// Returns whether or not the music volume is muted.
    ///
    /// This is equivalent to calling `self.music_volume.is_muted()`.
    pub fn is_music_volume_muted(&self) -> bool {
        self.music_volume.is_muted()
    }

    /// Returns whether or not the sound effects volume is muted.
    ///
    /// This is equivalent to calling `self.sound_effects_volume.is_muted()`.
    pub fn is_sound_effects_volume_muted(&self) -> bool {
        self.sound_effects_volume.is_muted()
    }

    /// Mutes the game volume.
    ///
    /// This is equivalent to calling `self.game_volume.mute()`.
    pub fn mute_game_volume(&mut self) {
        self.game_volume.mute();
    }

    /// Mutes the music volume.
    ///
    /// This is equivalent to calling `self.music_volume.mute()`.
    pub fn mute_music_volume(&mut self) {
        self.music_volume.mute();
    }

    /// Mutes the sound effects volume.
    ///
    /// This is equivalent to calling `self.sound_effects_volume.mute()`.
    pub fn mute_sound_effects_volume(&mut self) {
        self.sound_effects_volume.mute();
    }

    /// Unmutes the game volume.
    ///
    /// This is equivalent to calling `self.game_volume.unmute()`.
    pub fn unmute_game_volume(&mut self) {
        self.game_volume.unmute();
    }

    /// Unmutes the music volume.
    ///
    /// This is equivalent to calling `self.music_volume.unmute()`.
    pub fn unmute_music_volume(&mut self) {
        self.music_volume.unmute();
    }

    /// Unmutes the sound effects volume.
    ///
    /// This is equivalent to calling `self.sound_effects_volume.unmute()`.
    pub fn unmute_sound_effects_volume(&mut self) {
        self.sound_effects_volume.unmute();
    }
}
