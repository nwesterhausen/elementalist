use bevy::{app::AppExit, prelude::*};
use bevy_pkv::PkvStore;

use crate::GameState;

use super::{AudioSettingsBundle, VideoSettings};

/// Key that audio settings are stored under
const AUDIO_SETTINGS_KEY: &str = "audio_settings";
/// Key that video settings are stored under
const VIDEO_SETTINGS_KEY: &str = "video_settings";

#[derive(Component)]
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Attempt to load settings from the persistent key-value store
            .add_systems(Startup, load_settings);
        //Todo: somehow add a system that will save settings on demand?
        // Save settings when exiting the game via DROP trait?
    }
}

fn load_settings(mut commands: Commands, mut store: ResMut<PkvStore>) {
    // Load video settings
    if let Ok(quality) = store.get::<VideoSettings::DisplayQuality>(VIDEO_SETTINGS_KEY) {
        commands.insert_resource(quality);
        tracing::info!("Loaded video settings: {:?}", quality)
    } else {
        // If there are no video settings in the store, insert the default ones
        commands.insert_resource(VideoSettings::DisplayQuality::default());
    }

    // Load audio settings
    if let Ok(audio_settings) = store.get::<AudioSettingsBundle>(AUDIO_SETTINGS_KEY) {
        commands.insert_resource(audio_settings);
        tracing::info!("Loaded audio settings: {:?}", audio_settings)
    } else {
        // If there are no audio settings in the store, insert the default ones
        commands.insert_resource(AudioSettingsBundle::default());
    }

    // Load controls settings
    // if let Ok(Some(controls_settings)) = store.get::<settings::ControlsSettingsBundle>() {
    //     // Remove the default / previous controls settings before inserting the new ones
    //     commands.remove_resource::<settings::ControlsSettingsBundle>();
    //     commands.insert_resource(controls_settings);
    // } else {
    //     // If there are no controls settings in the store, insert the default ones
    //     commands.insert_resource(settings::ControlsSettingsBundle::default());
    // }
}

fn save_settings(
    mut store: ResMut<PkvStore>,
    video_settings: Res<VideoSettings::DisplayQuality>,
    audio_settings: Res<AudioSettingsBundle>,
) {
    // Save video settings
    match store.set(VIDEO_SETTINGS_KEY, video_settings.as_ref()) {
        Ok(_) => tracing::info!("Saved video settings: {:?}", video_settings),
        Err(e) => tracing::error!("Failed to save video settings: {:?}", e),
    }

    // Save audio settings
    match store.set(AUDIO_SETTINGS_KEY, audio_settings.as_ref()) {
        Ok(_) => tracing::info!("Saved audio settings: {:?}", audio_settings),
        Err(e) => tracing::error!("Failed to save audio settings: {:?}", e),
    }
}
