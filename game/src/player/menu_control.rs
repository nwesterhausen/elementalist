use bevy::prelude::*;
use game_library::state::{Overlay, Settings};
use leafwing_input_manager::action_state::ActionState;

use crate::{events::MenuInteraction, player::Player};

/// Handle menu input
pub fn menu_input(
    query: Query<&ActionState<MenuInteraction>, With<Player>>,
    menu_state: Res<State<Settings>>,
    mut next_menu_state: ResMut<NextState<Settings>>,
    mut next_overlay_state: ResMut<NextState<Overlay>>,
) {
    let action_state = query.single();
    if action_state.just_pressed(MenuInteraction::Up) {
        tracing::debug!("Up (in Menu)");
    }
    if action_state.just_pressed(MenuInteraction::Down) {
        tracing::debug!("Down (in Menu)");
    }
    if action_state.just_pressed(MenuInteraction::Select) {
        tracing::debug!("Select (in Menu)");
    }
    if action_state.just_pressed(MenuInteraction::Back) {
        if *menu_state == Settings::Main {
            next_overlay_state.set(Overlay::None);
            next_menu_state.set(Settings::Disabled);
        } else {
            next_menu_state.set(Settings::Main);
        }
    }
}
