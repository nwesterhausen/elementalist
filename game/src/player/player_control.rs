use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::{events::PlayerAction, player::Player, resources::OffsetCursorPosition};

/// Handle player input
pub fn player_control_system(
    query: Query<&ActionState<PlayerAction>, With<Player>>,
    cursor_position: Res<OffsetCursorPosition>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let action_state = query.single();
    if action_state.pressed(PlayerAction::Look) {
        if let Some(axis_pair) = action_state.clamped_axis_pair(PlayerAction::Look) {
            println!("Look: {:?}", axis_pair);
        } else {
            println!("Look");
        }
    }
    if action_state.just_pressed(PlayerAction::CastPrimary) {
        // Draw a sprite at the cursor position
        commands.spawn(SpriteBundle {
            transform: Transform::from_xyz(cursor_position.x, cursor_position.y, 0.0),
            texture: asset_server.load("firebolt.png").into(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(8.0, 8.0)),
                ..Default::default()
            },
            ..Default::default()
        });
    }
    if action_state.just_pressed(PlayerAction::CastSecondary) {
        // Draw a sprite at the cursor position
        commands.spawn(SpriteBundle {
            transform: Transform::from_xyz(cursor_position.x, cursor_position.y, 0.0),
            texture: asset_server.load("firebolt.png").into(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(8.0, 8.0)),
                ..Default::default()
            },
            ..Default::default()
        });
    }
    if action_state.just_pressed(PlayerAction::CastDefensive) {
        println!("CastDefensive");
    }
    if action_state.just_pressed(PlayerAction::CastUltimate) {
        println!("CastUltimate");
    }
    if action_state.just_pressed(PlayerAction::ToggleAutoCast) {
        println!("ToggleAutoCast");
    }
    if action_state.just_pressed(PlayerAction::ToggleAutoAim) {
        println!("ToggleAutoAim");
    }
    if action_state.just_pressed(PlayerAction::Interact) {
        println!("Interact");
    }
}
