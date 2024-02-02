use bevy::prelude::*;

use super::MenuState;
use crate::resources::ReturnToState;
use game_library::state::AppState;

/// All of the various "buttons" that can be clicked in any of the main menu screens
#[derive(Component, Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum ButtonAction {
    StartGame,
    Settings,
    Quit,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct MainMenuButton;

/// System to handle the main menu button actions
///
/// * `interaction_query`: grabs all the buttons that have been interacted with, with the components
///    Interaction and `ButtonAction` that have a changed interaction value (i.e. the button has been
///   pressed)
/// * `app_exit_events`: can be used to send an `AppExit` event to exit the game
/// * `menu_state(next)`: lets us change the menu state for the next frame
/// * `game_state(next)`: lets us change the game state for the next frame
#[allow(clippy::type_complexity)]
pub fn menu_actions(
    interaction_query: Query<
        (&Interaction, &ButtonAction),
        (Changed<Interaction>, With<Button>, With<MainMenuButton>),
    >,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<AppState>>,
    mut return_to_state: ResMut<ReturnToState>,
) {
    // Loop through all the buttons that have been interacted with
    for (interaction, menu_button_action) in &interaction_query {
        // If the button has been pressed, match the button action
        if *interaction == Interaction::Pressed {
            // Check which button action has been pressed (i.e. what action we attached to the button)
            match menu_button_action {
                ButtonAction::Quit => game_state.set(AppState::CleanUp),
                ButtonAction::StartGame => {
                    game_state.set(AppState::InGame);
                    menu_state.set(MenuState::Disabled);
                }
                ButtonAction::Settings => {
                    // Set the return to state to the main menu
                    return_to_state.0 = AppState::MainMenu;
                    // Set the game state to the settings menu
                    game_state.set(AppState::SettingsMenu);
                    menu_state.set(MenuState::Disabled);
                }
            }
        }
    }
}
