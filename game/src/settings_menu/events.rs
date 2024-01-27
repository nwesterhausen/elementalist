//! Events for changing the settings.

use bevy::prelude::*;

/// Events that can be sent to change the settings.
#[derive(Debug)]
pub(super) enum IndividualSetting {
    /// Change the font family.
    FontFamily,
    /// Change the auto-cast setting.
    AutoCast,
    /// Change the auto-aim setting.
    AutoAim,
    /// Change the main volume.
    MainVolume,
    /// Change the music volume.
    MusicVolume,
    /// Change the sound effects volume.
    SoundEffectsVolume,
}

/// Changed event for settings.
///
/// This simplifies the "button action" system, since we can just send this event and then handle it
/// in another system.
#[derive(Event, Debug)]
pub(super) struct ChangeSetting {
    /// The setting to change
    pub setting: IndividualSetting,
}
