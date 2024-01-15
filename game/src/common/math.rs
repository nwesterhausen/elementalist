use bevy::{math::Vec2, transform::components::Transform};

use crate::resources::CursorPosition;

pub fn slope_vec(player_transform: &Transform, cursor_position: &CursorPosition) -> Vec2 {
    let player_xy = Vec2::new(
        player_transform.translation.x,
        player_transform.translation.y,
    );
    let slope_vec = cursor_position.position - player_xy;
    slope_vec.normalize()
}
