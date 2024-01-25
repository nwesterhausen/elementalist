//! The plugin which adds the necessary states and systems to the app for
//! the settings menu to work.

use bevy::prelude::*;

use super::{
    base::{cleanup_menu_entities, clear_background},
    state::MenuState,
};

/// The settings menu plugin.
pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>();
        // Whatever we do to blur the background should happen (constantly?) while
        // the menu is not disabled. This has its own state check, so it should
        // be fine to run it constantly.
        app.add_systems(OnEnter(MenuState::Main), clear_background);
        app.add_systems(OnEnter(MenuState::Disabled), cleanup_menu_entities);
        // Then we should have systems for each of the menu states.
    }
}
