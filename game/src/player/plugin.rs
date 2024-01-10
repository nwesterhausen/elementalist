use bevy::prelude::*;

use super::{player_control, player_creation, player_sprite};
use crate::{resources, AppState};

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
                    player_control::player_control_system
                        .after(resources::update_cursor_position_resource),
                    player_control::menu_input,
                ),
            )
            // Sprite stuff
            .add_systems(OnEnter(AppState::InGame), player_sprite::setup_sprite)
            .add_systems(
                Update,
                (player_sprite::sprite_movement).run_if(in_state(AppState::InGame)),
            );
    }
}
