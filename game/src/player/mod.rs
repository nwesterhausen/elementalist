//! Player module. Contains all the player related code.
//!
//! This module contains all the code related to the player. This includes the player's avatar, the
//! player's controller, and the player's plugin. The player's plugin is used to add all the
//! systems and components for the player. The player's avatar is the entity that the player
//! controls. The player's controller is the input manager that the player uses to control the
//! player's avatar.

mod avatar;
mod bundle;
mod menu_control;
mod movement;
mod player_control;
mod player_creation;
mod player_sprite;
mod plugin;
mod animation;

pub use bundle::Player;
pub use plugin::PlayerPlugin;
