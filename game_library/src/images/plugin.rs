//! Plugin for the image modules (animations, etc)

use bevy::prelude::*;

use crate::state::Game;

use super::animation_system::{animate_sprites, draw_animated_frames};

/// Plugin for the image modules (animations, etc)
#[allow(clippy::module_name_repetitions)]
pub struct ImagesPlugin;

impl Plugin for ImagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (animate_sprites, draw_animated_frames).run_if(in_state(Game::Playing)),
        );
    }
}
