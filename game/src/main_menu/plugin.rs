use bevy::prelude::*;

use crate::despawn_with_tag;
use game_library::state::AppState;

use super::{button_actions::menu_actions, screens};

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    #[default]
    Disabled,
}

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
pub struct OnMainMenuScreen;

/// System to setup the main menu
///
/// When the main menu is entered, we setup the main menu state to the main menu.
fn transition_to_main_menu(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

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
            .add_systems(Update, menu_actions.run_if(in_state(AppState::MainMenu)))
            // Main main screen
            .add_systems(OnEnter(MenuState::Main), screens::main_menu)
            .add_systems(
                OnExit(MenuState::Main),
                despawn_with_tag::<OnMainMenuScreen>,
            );
    }
}
