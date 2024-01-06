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

mod app_info;
mod app_state;
mod app_systems;
mod main_menu;
mod splash_screen;

pub use app_state::AppState;
pub use app_systems::despawn_screen;
use bevy_pkv::PkvStore;

fn main() {
    App::new()
        // The clear color is the color the screen is cleared to before each frame is drawn
        .insert_resource(ClearColor(Color::BLACK))
        // Set basic window properties
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: app_info::game_name_and_version().to_string(),
                resolution: (1280.0, 720.0).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        }))
        // Add state
        .add_state::<AppState>()
        // Add a persistent key-value store for settings, etc.
        .insert_resource(PkvStore::new("nwest.games", "elementalist"))
        // Add Camera
        .add_systems(Startup, app_systems::setup_camera)
        // Add plugins for splash screen and menu
        .add_plugins((splash_screen::SplashScreenPlugin, main_menu::MainMenuPlugin))
        // Launch
        .run();
}
