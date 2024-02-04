use bevy::prelude::*;
use game_library::state::{AppState, MenuState};
use leafwing_input_manager::action_state::ActionState;

use crate::{events::MenuInteraction, player::Player, resources::ReturnToState};

/// Handle menu input
pub fn menu_input(
    query: Query<&ActionState<MenuInteraction>, With<Player>>,
    menu_state: Res<State<MenuState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    return_to_state: Res<ReturnToState>,
) {
    let action_state = query.single();
    if action_state.just_pressed(MenuInteraction::Up) {
        println!("Up (in Menu)");
    }
    if action_state.just_pressed(MenuInteraction::Down) {
        println!("Down (in Menu)");
    }
    if action_state.just_pressed(MenuInteraction::Select) {
        println!("Select (in Menu)");
    }
    if action_state.just_pressed(MenuInteraction::Back) {
        if *menu_state == MenuState::Main {
            next_app_state.set(return_to_state.0.clone());
            next_menu_state.set(MenuState::Disabled);
        } else {
            next_menu_state.set(MenuState::Main);
        }
    }
}
