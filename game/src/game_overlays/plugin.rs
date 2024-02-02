use bevy::prelude::*;

use game_library::state::OverlayState;

use super::status_screen::StatusScreenPlugin;

/// Plugin for the status screen
pub struct GameOverlaysPlugin;

impl Plugin for GameOverlaysPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<OverlayState>()
            .add_plugins(StatusScreenPlugin);
    }
}
