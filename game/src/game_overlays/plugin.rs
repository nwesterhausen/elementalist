use bevy::prelude::*;

use crate::resources::OverlayState;

use super::status_screen::StatusScreenPlugin;

/// Plugin for the status screen
pub struct GameOverlaysPlugin;

impl Plugin for GameOverlaysPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<OverlayState>()
            .add_plugins(StatusScreenPlugin);
    }
}
