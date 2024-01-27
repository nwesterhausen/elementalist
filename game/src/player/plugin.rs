use bevy::prelude::*;

use super::{
    entity::{self, PlayerAvatar},
    menu_control, movement,
    player_control::PlayerControlsPlugin,
    player_creation,
};
use crate::{despawn_with_tag, resources::AppState};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Spawn player (user controller)
            .add_systems(Startup, player_creation::spawn_player_controller)
            // Handle player inputs
            .add_plugins(PlayerControlsPlugin)
            .add_systems(Update, (menu_control::menu_input,))
            // Sprite stuff
            .add_systems(OnEnter(AppState::InGame), entity::spawn_player)
            .add_systems(
                Update,
                (movement::player_movement_controls).run_if(in_state(AppState::InGame)),
            )
            // Remove player when leaving game
            .add_systems(
                OnExit(AppState::InGame),
                despawn_with_tag::<PlayerAvatar>.run_if(not(in_state(AppState::SettingsMenu))),
            );
    }
}
