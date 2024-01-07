use bevy::{app::AppExit, prelude::*};

use crate::AppState;

use super::{button_actions::ButtonAction, components::*, menu_state::MenuState};

/// System for changing button colors when hovered, etc
///
/// interaction_query: grabs all the buttons that have been interacted with, with the components
///   Interaction, children, and if they are a selected option (e.g. part of a radio group). It grabs
///   what has changed about the interaction (i.e. if it has changed at all)
/// text_query: let's us grab the text component of the button
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &Children, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    // Loop through all the buttons that have been interacted with (and all the components we grabbed)
    for (interaction, children, selected) in &mut interaction_query {
        // Try to grab the text component of the button, and if we can't we will just skip (continue)
        // We are changing the text color and not the button color because drawing big squares is dumb. :)
        let Ok(mut text) = text_query.get_mut(children[0]) else {
            tracing::error!("Button has no text (and was probably supposed to)");
            continue;
        };
        // Match the interaction and selected option to change the text color
        // i.e., hover, not-hover & selected, pressed, and not-pressed & not selected. (or close that that)
        match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => {
                text.sections[0].style.color = Color::DARK_GREEN;
            }
            (Interaction::Hovered, Some(_)) => text.sections[0].style.color = Color::YELLOW_GREEN,
            (Interaction::Hovered, None) => text.sections[0].style.color = Color::YELLOW,
            (Interaction::None, None) => text.sections[0].style.color = Color::WHITE,
        }
    }
}

/// System to handle the main menu button actions
///
/// interaction_query: grabs all the buttons that have been interacted with, with the components
///    Interaction and ButtonAction that have a changed interaction value (i.e. the button has been
///   pressed)
/// app_exit_events: can be used to send an AppExit event to exit the game
/// menu_state(next): lets us change the menu state for the next frame
/// game_state(next): lets us change the game state for the next frame
pub fn menu_actions(
    interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<AppState>>,
) {
    // Loop through all the buttons that have been interacted with
    for (interaction, menu_button_action) in &interaction_query {
        // If the button has been pressed, match the button action
        if *interaction == Interaction::Pressed {
            // Check which button action has been pressed (i.e. what action we attached to the button)
            match menu_button_action {
                ButtonAction::Quit => app_exit_events.send(AppExit),
                ButtonAction::StartGame => {
                    game_state.set(AppState::InGame);
                    menu_state.set(MenuState::Disabled);
                }
                ButtonAction::Settings => menu_state.set(MenuState::Settings),
                ButtonAction::BackToMenu => menu_state.set(MenuState::Main),
                ButtonAction::BackToSettings => menu_state.set(MenuState::Settings),
                ButtonAction::SettingsAudio => menu_state.set(MenuState::SettingsAudio),
                ButtonAction::SettingsVideo => menu_state.set(MenuState::SettingsVideo),
                ButtonAction::SettingsControls => menu_state.set(MenuState::SettingsControls),
                ButtonAction::SettingsGameplay => menu_state.set(MenuState::SettingsGameplay),
                // A catch-all for any button actions we haven't handled (yet)
                // _ => tracing::error!("Unhandled button action: {:?}", menu_button_action),
            }
        }
    }
}

/// System to setup the main menu
///
/// When the main menu is entered, we setup the main menu state to the main menu.
pub fn transition_to_main_menu(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}