use bevy::prelude::*;
use game_library::StatEnum;
use leafwing_input_manager::action_state::ActionState;

use crate::{
    common::{movement::Velocity, stats::StatBundle},
    events::PlayerAction,
    player::Player,
};

/// Handle player input for movement
pub fn player_movement_controls(
    mut query: Query<&mut Velocity, With<Player>>,
    action_query: Query<&ActionState<PlayerAction>, With<Player>>,
    stat_query: Query<&StatBundle, With<Player>>,
) {
    let Ok(action_state) = action_query.get_single() else {
        tracing::error!("player_movement_controls: failed to get action state");
        return;
    };

    let Ok(mut velocity) = query.get_single_mut() else {
        tracing::error!("player_movement_controls: failed to get velocity");
        return;
    };

    let Ok(stat_bundle) = stat_query.get_single() else {
        tracing::error!("player_movement_controls: failed to get stat_bundle");
        return;
    };

    let Some(speed) = stat_bundle.get_stat(StatEnum::MovementSpeed) else {
        tracing::error!("player_movement_controls: failed to get speed stat");
        return;
    };

    // Reset velocity to 0.0 because we're going to set it directly
    // We want the player to stop moving if they're not pressing the move button
    // Todo: This will be "jerky" and probably needs some tweaking to feel good.
    velocity.value = Vec2::ZERO;

    if action_state.pressed(PlayerAction::Move) {
        if let Some(axis_pair) = action_state.clamped_axis_pair(PlayerAction::Move) {
            velocity.value = axis_pair.xy().normalize() * speed.value();
        }
    }
}
