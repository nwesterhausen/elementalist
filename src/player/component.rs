use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::settings::{map_controls, Action};

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub input_manager: InputManagerBundle<Action>,
}

/// Spawn the player
pub fn spawn_player(mut commands: Commands) {
    commands.spawn(PlayerBundle {
        player: Player,
        input_manager: InputManagerBundle {
            input_map: map_controls(),
            ..Default::default()
        },
    });
}
