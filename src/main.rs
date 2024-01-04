//! # Elementalist
//!
//! A rogue-lite game where you play as an elementalist.

#![warn(
    missing_docs,
    unreachable_code,
    unreachable_patterns,
    clippy::unwrap_used,
    clippy::expect_used
)]
#![deny(unsafe_code)]

use bevy::prelude::*;

mod main_menu;
mod player;
mod settings;
mod splash;
mod state;

pub use state::GameState;

const TEXT_COLOR: Color = Color::WHITE;

fn main() {
    App::new()
        // Load plugins
        .add_plugins((DefaultPlugins))
        // Declare the game state (starting value is always DEFAULT)
        .add_state::<GameState>()
        // Setup a 2d camera
        .add_systems(Startup, setup_camera)
        // Add plugins for each state ?
        .add_plugins((splash::Plugin, main_menu::Plugin))
        // Run the app
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/// Despawn all entities with the given tag (component)
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
