use bevy::prelude::*;
use leafwing_input_manager::{action_state::ActionState, InputManagerBundle};

use crate::{
    events::{MenuInteraction, PlayerAction},
    player::Player,
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
