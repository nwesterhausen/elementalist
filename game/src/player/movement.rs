use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use game_library::{enums::StatEnum, StatBundle};
use leafwing_input_manager::action_state::ActionState;

use crate::{events::PlayerAction, player::Player};

use super::avatar::PlayerAvatar;

pub(super) const STANDING_FORWARD_IDX: usize = 3;
pub(super) const FACING_SIDEWAYS_IDX: usize = 0;

/// Handle player input for movement
pub fn player_movement_controls(
    mut query: Query<&mut KinematicCharacterController, With<Player>>,
    action_query: Query<&ActionState<PlayerAction>, With<Player>>,
    stat_query: Query<&StatBundle, With<Player>>,
    mut sprite_query: Query<&mut TextureAtlasSprite, With<PlayerAvatar>>,
) {
    let Ok(action_state) = action_query.get_single() else {
        tracing::error!("player_movement_controls: failed to get action state");
        return;
    };

    let Ok(stat_bundle) = stat_query.get_single() else {
        tracing::error!("player_movement_controls: failed to get stat_bundle");
        return;
    };

    let Some(speed) = stat_bundle.get_stat(&StatEnum::MovementSpeed) else {
        tracing::error!("player_movement_controls: failed to get speed stat");
        return;
    };

    let Ok(mut sprite) = sprite_query.get_single_mut() else {
        tracing::error!("player_movement_controls: failed to get player sprite");
        return;
    };

    // we expect just one `KinematicCharacterController` for the player
    let Ok(mut controller) = query.get_single_mut() else {
        tracing::error!("player_movement_controls: failed to get player controller");
        return;
    };

    if action_state.pressed(PlayerAction::Move) {
        if let Some(axis_pair) = action_state.clamped_axis_pair(PlayerAction::Move) {
            sprite.index = FACING_SIDEWAYS_IDX;
            controller.translation = Some(axis_pair.xy().normalize_or_zero() * (speed.value()));

            sprite.flip_x = axis_pair.x() < 0.0;
        }
    } else {
        sprite.index = STANDING_FORWARD_IDX;
    }
}

/// Plugin that adjust the player's z-index based on their y position
pub(super) fn update_player_z_index(mut query: Query<&mut Transform, With<PlayerAvatar>>) {
    for mut transform in &mut query {
        transform.translation.z = -transform.translation.y;
    }
}
