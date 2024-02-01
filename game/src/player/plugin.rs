use bevy::prelude::*;
use game_library::{Health, Xp};

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
            .add_systems(OnEnter(AppState::InGame), entity::spawn_player_avatar)
            .add_systems(
                Update,
                (movement::player_movement_controls).run_if(in_state(AppState::InGame)),
            )
            // Remove player when leaving game
            .add_systems(
                OnEnter(AppState::MainMenu),
                despawn_with_tag::<PlayerAvatar>,
            );

        // Testing stuff
        app.add_systems(Update, subtract_health);
    }
}

/// System that subtracts 1 from a the players health when the 'H' key is pressed.
fn subtract_health(
    mut player_health: Query<&mut Health, With<PlayerAvatar>>,
    mut player_xp: Query<&mut Xp, With<PlayerAvatar>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::H) {
        for mut health in &mut player_health {
            health.value -= 1;
        }
    }
    if keyboard_input.just_pressed(KeyCode::X) {
        for mut xp in &mut player_xp {
            xp.add(1);
        }
    }
}
