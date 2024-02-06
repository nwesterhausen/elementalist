//! The plugin which adds the necessary states and systems to the app for
//! the settings menu to work.

use bevy::prelude::*;

use crate::despawn_with_tag;
use game_library::state::{Overlay, Settings};

use super::{
    accessibility::AccessibilitySettingsMenuPlugin,
    audio::AudioSettingsMenuPlugin,
    base::{clear_background, transition_to_base_menu, SettingsMenuBackground, SettingsMenuEntity},
    button_actions::menu_actions,
    controls::ControlsSettingsMenuPlugin,
    display::DisplaySettingsMenuPlugin,
    events::ChangeSetting,
    gameplay::GameplaySettingsMenuPlugin,
    main::{show_main_menu, BaseSettingsMenuEntity},
};

/// The settings menu plugin.
pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<Settings>();
        app.add_event::<ChangeSetting>();
        // Add system to setup the menu
        app.add_systems(OnEnter(Overlay::Settings), transition_to_base_menu);
        app.add_systems(OnEnter(Settings::Main), (clear_background, show_main_menu));
        app.add_systems(
            OnExit(Settings::Main),
            (despawn_with_tag::<BaseSettingsMenuEntity>,),
        );
        // When disabled, we should clean up all the entities that are part of the menu.
        app.add_systems(
            OnEnter(Settings::Disabled),
            (
                despawn_with_tag::<SettingsMenuEntity>,
                despawn_with_tag::<SettingsMenuBackground>,
            ),
        );
        // Each "page" of the settings menu has its own plugin that adds the necessary systems
        app.add_plugins((
            AccessibilitySettingsMenuPlugin,
            AudioSettingsMenuPlugin,
            ControlsSettingsMenuPlugin,
            DisplaySettingsMenuPlugin,
            GameplaySettingsMenuPlugin,
        ));
        // Add system to update the buttons on hover, and respond to button presses
        app.add_systems(Update, menu_actions.run_if(in_state(Overlay::Settings)));
    }
}
