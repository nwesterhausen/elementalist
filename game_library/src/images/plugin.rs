//! Plugin for the image modules (animations, etc)

use bevy::prelude::*;

use crate::state::Game;

use super::animation_system::{draw_sprites, update_current_animation, update_tile_atlas_textures};

/// Plugin for the image modules (animations, etc)
///
/// Runs the systems for image animations and updates.
///
/// 1. `draw_sprites` - Draws the sprites on the screen. This advances the animation frames and adjust the facing direction of the sprite.
/// 2. `transition_to_idle` - Transitions the sprite to the idle animation if the casting or attack animation finishes.
///
/// These only run if the game state is `Game::Playing`.
#[allow(clippy::module_name_repetitions)]
pub struct ImagesPlugin;

impl Plugin for ImagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_tile_atlas_textures,
                draw_sprites,
                update_current_animation,
            )
                .run_if(in_state(Game::Playing)),
        );
    }
}
