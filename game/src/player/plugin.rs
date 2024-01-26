use bevy::prelude::*;

use super::{entity, menu_control, movement, player_control, player_creation};
use crate::resources::{self, AppState};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Spawn player (user controller)
            .add_systems(Startup, player_creation::spawn_player_controller)
            // Handle player inputs
            .add_systems(
                Update,
                (
                    player_control::player_control_system.after(resources::update_cursor_position),
                    menu_control::menu_input,
                ),
            )
            // Sprite stuff
            .add_systems(OnEnter(AppState::InGame), entity::spawn_player)
            .add_systems(
                Update,
                (movement::player_movement_controls).run_if(in_state(AppState::InGame)),
            );
    }
}
