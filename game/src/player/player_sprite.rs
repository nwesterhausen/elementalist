use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use super::{Player, MOVE_SPEED};
use crate::events::PlayerAction;

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
