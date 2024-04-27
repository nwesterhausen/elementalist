use bevy::prelude::*;

use super::MenuState;
use elementalist_game_library::{
    buttons::ButtonSystem,
    state::{AppState, Overlay},
};

/// Component to identify buttons that are part of the main menu.
#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct MainMenuButton;

/// System to handle pressing the Start Game button in the main menu.
pub(super) fn start_game_oneshot(
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<AppState>>,
) {
    game_state.set(AppState::InGame);
    menu_state.set(MenuState::Disabled);
}

/// System to handle pressing the Settings button in the main menu.
pub(super) fn settings_oneshot(mut overlay_state: ResMut<NextState<Overlay>>) {
    overlay_state.set(Overlay::Settings);
}

/// System to handle pressing the Quit button in the main menu.
pub(super) fn quit_button_oneshot(mut game_state: ResMut<NextState<AppState>>) {
    game_state.set(AppState::CleanUp)
}

/// Resource for storing the buttons for future reference, instead of re-registering the systems
/// whenever we enter the main menu.
#[derive(Resource)]
pub(super) struct MenuButtons {
    pub start: ButtonSystem,
    pub settings: ButtonSystem,
    pub quit: ButtonSystem,
}

/// System to register the buttons for future reference.
///
/// This inserts a resource with the button systems, which can be used to handle button presses in other systems.
pub(super) fn register_buttons(world: &mut World) {
    let start = ButtonSystem {
        pressed_event: Some(world.register_system(start_game_oneshot)),
    };
    let settings = ButtonSystem {
        pressed_event: Some(world.register_system(settings_oneshot)),
    };
    let quit = ButtonSystem {
        pressed_event: Some(world.register_system(quit_button_oneshot)),
    };
    world.insert_resource(MenuButtons {
        start,
        settings,
        quit,
    });
}
