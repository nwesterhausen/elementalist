use bevy::{app::AppExit, prelude::*};

use crate::AppState;

use super::{button_actions::ButtonAction, components::*, menu_state::MenuState};

/// System for changing button colors when hovered, etc
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &Children, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, children, selected) in &mut interaction_query {
        let Ok(mut text) = text_query.get_mut(children[0]) else {
            tracing::error!("Button has no text");
            continue;
        };
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
pub fn menu_actions(
    interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                ButtonAction::Quit => app_exit_events.send(AppExit),
                ButtonAction::StartGame => {
                    game_state.set(AppState::InGame);
                    menu_state.set(MenuState::Disabled);
                }
                ButtonAction::Settings => menu_state.set(MenuState::Settings),
                _ => tracing::error!("Unhandled button action: {:?}", menu_button_action),
            }
        }
    }
}
