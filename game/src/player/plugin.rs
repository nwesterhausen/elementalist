use bevy::prelude::*;
use game_library::{state::Game, Health, Xp};

use super::{
    animation,
    avatar::{self, PlayerAvatar},
    menu_control, movement,
    player_control::PlayerControlsPlugin,
    player_creation,
};
use crate::{camera::MainCamera, despawn_with_tag};

/// Plugin which handles adding all the systems and components for the player.
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
            .add_systems(OnEnter(Game::Playing), avatar::spawn_player_avatar)
            .add_systems(
                Update,
                (
                    movement::player_movement_controls,
                    movement::update_player_z_index,
                    animation::set_casting_animation,
                )
                    .run_if(in_state(Game::Playing)),
            )
            // Remove player when leaving game
            .add_systems(OnExit(Game::Playing), despawn_with_tag::<PlayerAvatar>);

        // Testing stuff
        app.add_systems(Update, subtract_health)
            .add_systems(PostUpdate, camera_movement.run_if(in_state(Game::Playing)));
    }
}

fn camera_movement(
    mut camera: Query<&mut Transform, (With<MainCamera>, Without<PlayerAvatar>)>,
    player: Query<&Transform, With<PlayerAvatar>>,
) {
    for mut transform in &mut camera {
        for player_transform in &player {
            transform.translation.x = player_transform.translation.x;
            transform.translation.y = player_transform.translation.y;
        }
    }
}

/// System that subtracts 1 from a the players health when the 'H' key is pressed.
///
/// It also adds 1 to the player's experience points when the 'X' key is pressed.
///
/// This is just a test system to see if the player's health and experience points are
/// being updated correctly.
fn subtract_health(
    mut player_health: Query<&mut Health, With<PlayerAvatar>>,
    mut player_xp: Query<&mut Xp, With<PlayerAvatar>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyH) {
        for mut health in &mut player_health {
            health.value -= 1;
        }
    }
    if keyboard_input.just_pressed(KeyCode::KeyX) {
        for mut xp in &mut player_xp {
            xp.add(1);
        }
    }
}
