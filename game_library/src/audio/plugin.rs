//! Plugin for audio. Adds any systems, events, resources used for our audio system.
use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin as KiraAudioPlugin;

/// Audio plugin
#[allow(clippy::module_name_repetitions)]
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        // Add the Kira audio plugin
        app.add_plugins(KiraAudioPlugin);
        // Add our systems, etc
    }
}
