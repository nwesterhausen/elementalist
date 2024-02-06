use bevy::prelude::*;
use game_library::state::{AppState, Game};

/// Plugin and control flow for the settings menu.
pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), trigger_generate);
    }
}

fn trigger_generate(mut next_game_state: ResMut<NextState<Game>>) {
    tracing::info!("Triggering game generation");
    next_game_state.set(Game::Generating);
}
