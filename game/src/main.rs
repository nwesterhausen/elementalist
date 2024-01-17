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
// Hide the console window on Windows when not in debug mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

mod app_info;
mod app_state;
mod app_systems;
mod camera;
mod common;
mod dev_systems;
mod events;
mod game;
mod main_menu;
mod player;
mod resources;
mod spells;
mod splash_screen;

pub use app_state::AppState;
pub use app_systems::despawn_screen;
use events::{MenuInteraction, PlayerAction};

fn main() {
    App::new()
        // The clear color is the color the screen is cleared to before each frame is drawn
        .insert_resource(ClearColor(Color::DARK_GREEN))
        // Set basic window properties
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
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
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, app_systems::set_window_icon)
        // Add our dev-mode systems (they disable themselves in release mode)
        .add_plugins(dev_systems::DevSystemsPlugin)
        // Add state
        .add_state::<AppState>()
        // Add all the general resources and their update systems (e.g. cursor position)
        .add_plugins(resources::ElementalistResourcesPlugin)
        .add_systems(
            Startup,
            (
                app_systems::add_game_descriptor,
                app_systems::load_data_file_dir,
            ),
        )
        // Add input processing
        .add_plugins((
            InputManagerPlugin::<PlayerAction>::default(),
            InputManagerPlugin::<MenuInteraction>::default(),
        ))
        // Add camera plugin
        .add_plugins(camera::CameraPlugin)
        // Add our plugins for the menu screen and the splash screen
        .add_plugins((splash_screen::SplashScreenPlugin, main_menu::MainMenuPlugin))
        // Add the game plugin
        .add_plugins(game::GamePlugin)
        // Add the player plugin
        .add_plugins(player::PlayerPlugin)
        // Add the spells plugin
        .add_plugins(spells::SpellsPlugin)
        // Add the movement plugin
        .add_plugins(common::movement::MovementPlugin)
        // Launch
        .run();
}
