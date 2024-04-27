use bevy::prelude::*;
use elementalist_game_library::state::{AppState, Game};

/// Plugin and control flow for the settings menu.
pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), trigger_generate);
        app.add_systems(
            OnEnter(AppState::MainMenu),
            trigger_disabled.run_if(not(in_state(Game::Disabled))),
        );
    }
}

fn trigger_generate(mut next_game_state: ResMut<NextState<Game>>) {
    tracing::info!("Triggering game generation");
    next_game_state.set(Game::Generating);
}

fn trigger_disabled(mut next_game_state: ResMut<NextState<Game>>) {
    tracing::info!("Triggering game disabled");
    next_game_state.set(Game::Disabled);
}
