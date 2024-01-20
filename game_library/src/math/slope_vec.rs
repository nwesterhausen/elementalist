use bevy::{math::Vec2, transform::components::Transform};

use crate::CursorPosition;

/// Returns a vector representing the slope between the player and the cursor
///
/// # Arguments
///
/// * `player_transform` - The transform of the player
/// * `cursor_position` - The position of the cursor (from the cursor resource)
///
/// # Returns
///
/// * `Vec2` - The slope vector (normalized)
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn slope_vec(player_transform: &Transform, cursor_position: &CursorPosition) -> Vec2 {
    let player_xy = Vec2::new(
        player_transform.translation.x,
        player_transform.translation.y,
    );
    let slope_vec = cursor_position.position - player_xy;
    slope_vec.normalize()
}
