use bevy::prelude::*;

use game_library::state::Overlay;

use super::{skill_book::SkillBookUiPlugin, status_screen::StatusScreenPlugin};

/// Plugin for the status screen
pub struct GameOverlaysPlugin;

impl Plugin for GameOverlaysPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<Overlay>()
            .add_plugins((StatusScreenPlugin, SkillBookUiPlugin));
    }
}
