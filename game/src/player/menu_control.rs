use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::{events::MenuInteraction, player::Player};

/// Handle menu input
pub fn menu_input(query: Query<&ActionState<MenuInteraction>, With<Player>>) {
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
        println!("Back (in Menu)");
    }
}
