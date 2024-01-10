mod entity;
mod menu_control;
mod movement;
mod player_control;
mod player_creation;
mod player_sprite;
mod plugin;

pub use entity::Player;
pub use plugin::PlayerPlugin;

const BASE_MOVE_SPEED: f32 = 100.0;
