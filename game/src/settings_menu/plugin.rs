//! The plugin which adds the necessary states and systems to the app for
//! the settings menu to work.

use bevy::prelude::*;

use crate::{despawn_with_tag, AppState};

use super::{
    accessibility::{show_accessibility_settings, AccessibilitySettingsMenuEntity},
    audio::{show_audio_settings, AudioSettingsMenuEntity},
    base::{clear_background, transition_to_base_menu, MenuBackground, MenuEntity},
    button_actions::{button_system, menu_actions},
    controls::{show_controls_settings, ControlsSettingsMenuEntity},
    display::{show_display_settings, DisplaySettingsMenuEntity},
    gameplay::{show_gameplay_settings, GameplaySettingsMenuEntity},
    main::{show_main_menu, MainMenuEntity},
    state::MenuState,
};

/// The settings menu plugin.
pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>();
        // Add system to setup the menu
        app.add_systems(OnEnter(AppState::SettingsMenu), transition_to_base_menu);
        app.add_systems(OnEnter(MenuState::Main), (clear_background, show_main_menu));
        app.add_systems(
            OnExit(MenuState::Main),
            (
                despawn_with_tag::<MainMenuEntity>,
                despawn_with_tag::<AudioSettingsMenuEntity>,
                despawn_with_tag::<DisplaySettingsMenuEntity>,
                despawn_with_tag::<ControlsSettingsMenuEntity>,
                despawn_with_tag::<GameplaySettingsMenuEntity>,
                despawn_with_tag::<AccessibilitySettingsMenuEntity>,
            ),
        );
        // When disabled, we should clean up all the entities that are part of the menu.
        app.add_systems(
            OnEnter(MenuState::Disabled),
            (
                despawn_with_tag::<MenuEntity>,
                despawn_with_tag::<MenuBackground>,
                despawn_with_tag::<MainMenuEntity>,
                despawn_with_tag::<AudioSettingsMenuEntity>,
                despawn_with_tag::<DisplaySettingsMenuEntity>,
                despawn_with_tag::<ControlsSettingsMenuEntity>,
                despawn_with_tag::<GameplaySettingsMenuEntity>,
                despawn_with_tag::<AccessibilitySettingsMenuEntity>,
            ),
        );
        // Then we should have systems for each of the menu states (display, audio, controls, gameplay)
        app.add_systems(OnEnter(MenuState::Display), show_display_settings);
        app.add_systems(
            OnExit(MenuState::Display),
            despawn_with_tag::<DisplaySettingsMenuEntity>,
        );
        app.add_systems(OnEnter(MenuState::Audio), show_audio_settings);
        app.add_systems(
            OnExit(MenuState::Audio),
            despawn_with_tag::<AudioSettingsMenuEntity>,
        );
        app.add_systems(OnEnter(MenuState::Controls), show_controls_settings);
        app.add_systems(
            OnExit(MenuState::Controls),
            despawn_with_tag::<ControlsSettingsMenuEntity>,
        );
        app.add_systems(OnEnter(MenuState::Gameplay), show_gameplay_settings);
        app.add_systems(
            OnExit(MenuState::Gameplay),
            despawn_with_tag::<GameplaySettingsMenuEntity>,
        );
        app.add_systems(
            OnEnter(MenuState::Accessibility),
            show_accessibility_settings,
        );
        app.add_systems(
            OnExit(MenuState::Accessibility),
            despawn_with_tag::<AccessibilitySettingsMenuEntity>,
        );
        // Add system to update the buttons on hover, etc
        app.add_systems(
            Update,
            (button_system, menu_actions).run_if(in_state(AppState::SettingsMenu)),
        );
    }
}
