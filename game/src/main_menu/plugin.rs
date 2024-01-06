use bevy::prelude::*;

use crate::{app_systems, AppState};

use super::{
    menu_state::MenuState,
    scenes,
    systems::{button_system, menu_actions},
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
            // Add the main menu screen
            .add_systems(
                OnEnter(AppState::MainMenu),
                (scenes::main_menu, app_systems::add_game_descriptor),
            )
            // Add system to update the buttons on hover, etc
            .add_systems(
                Update,
                (button_system, menu_actions).run_if(in_state(AppState::MainMenu)),
            );
    }
}
