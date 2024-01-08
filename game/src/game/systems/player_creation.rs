use bevy::prelude::*;
use leafwing_input_manager::{action_state::ActionState, InputManagerBundle};

use crate::{
    entities::Player,
    events::{MenuInteraction, PlayerAction},
};

/// Spawn a player controller (sets up input processing)
pub fn spawn_player_controller(mut commands: Commands) {
    commands
        .spawn(InputManagerBundle::<PlayerAction> {
            // What actions are currently pressed
            action_state: ActionState::default(),
            // Describes how to convert input into actions
            input_map: PlayerAction::default_input_map(),
        })
        // Add the player component
        .insert(Player);
    commands
        .spawn(InputManagerBundle::<MenuInteraction> {
            // What actions are currently pressed
            action_state: ActionState::default(),
            // Describes how to convert input into actions
            input_map: MenuInteraction::default_input_map(),
        })
        // Add the player component
        .insert(Player);
}

/// Sprite setup
pub fn setup_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("panda.png").into(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Player);
}

const MOVE_SPEED: f32 = 100.0;

/// Move sprite around via translation
pub fn sprite_movement(
    time: Res<Time>,
    mut transform: Query<&mut Transform, With<Player>>,
    action: Query<&ActionState<PlayerAction>, With<Player>>,
) {
    let action_state = action.single();
    let mut transform = transform.single_mut();
    if action_state.pressed(PlayerAction::Move) {
        if let Some(axis_pair) = action_state.clamped_axis_pair(PlayerAction::Move) {
            transform.translation.x += axis_pair.x() * time.delta_seconds() * MOVE_SPEED;
            transform.translation.y += axis_pair.y() * time.delta_seconds() * MOVE_SPEED;
        }
    }
}
