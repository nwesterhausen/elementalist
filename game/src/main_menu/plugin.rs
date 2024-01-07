use bevy::prelude::*;

use crate::{despawn_screen, AppState};

use super::{
    components,
    menu_state::MenuState,
    scenes,
    systems::{button_system, menu_actions, transition_to_main_menu},
};

/// The main menu plugin. This plugin is responsible for setting up the main menu.
///
/// It includes the following screens:
///
/// * Main menu (continue, new game, settings, quit)
/// * Settings (audio, video, controls, gameplay, back)
/// * Audio settings
/// * Video settings
/// * Controls settings
/// * Gameplay settings
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add a resource to track which menu state we are in
            .add_state::<MenuState>()
            // Transition to the main menu when entering the main menu state (starts tracking our MenuState at Main)
            .add_systems(OnEnter(AppState::MainMenu), transition_to_main_menu)
            // Add system to update the buttons on hover, etc
            .add_systems(
                Update,
                (button_system, menu_actions).run_if(in_state(AppState::MainMenu)),
            )
            // Main main screen
            .add_systems(OnEnter(MenuState::Main), scenes::main_menu)
            .add_systems(
                OnExit(MenuState::Main),
                despawn_screen::<components::OnMainMenuScreen>,
            )
            // Settings screen
            .add_systems(OnEnter(MenuState::Settings), scenes::settings)
            .add_systems(
                OnExit(MenuState::Settings),
                despawn_screen::<components::OnSettingsMenuScreen>,
            )
            // Audio settings screen
            .add_systems(OnEnter(MenuState::SettingsAudio), scenes::audio_settings)
            .add_systems(
                OnExit(MenuState::SettingsAudio),
                despawn_screen::<components::OnAudioSettingsMenuScreen>,
            )
            // Video settings screen
            .add_systems(OnEnter(MenuState::SettingsVideo), scenes::video_settings)
            .add_systems(
                OnExit(MenuState::SettingsVideo),
                despawn_screen::<components::OnVideoSettingsMenuScreen>,
            )
            // Controls settings screen
            .add_systems(
                OnEnter(MenuState::SettingsControls),
                scenes::controls_settings,
            )
            .add_systems(
                OnExit(MenuState::SettingsControls),
                despawn_screen::<components::OnControlsSettingsMenuScreen>,
            )
            // Gameplay settings screen
            .add_systems(
                OnEnter(MenuState::SettingsGameplay),
                scenes::gameplay_settings,
            )
            .add_systems(
                OnExit(MenuState::SettingsGameplay),
                despawn_screen::<components::OnGameplaySettingsMenuScreen>,
            );
    }
}