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

mod game;
mod menu;
mod player;
mod settings;
mod splash;

const TEXT_COLOR: Color = Color::WHITE;

/// Global game state enum
#[derive(Clone, Copy, Default, Eq, PartialEq, Hash, Debug, States)]
enum GameState {
    #[default]
    /// Splash screen shown when loading the game assets, et al.
    Splash,
    /// The main menu
    MainMenu,
    /// Inside the game (a loaded game)
    Game,
    /// A specific run as part of the game (tracks progression on a wider scale)
    SingleRun,
}

fn main() {
    App::new()
        // Load plugins
        .add_plugins((DefaultPlugins, settings::SettingsPlugin))
        // Declare the game state (starting value is always DEFAULT)
        .add_state::<GameState>()
        // Setup a 2d camera
        .add_systems(Startup, setup_camera)
        // Add plugins for each state ?
        .add_plugins((splash::SplashPlugin, menu::MenuPlugin, game::GamePlugin))
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
