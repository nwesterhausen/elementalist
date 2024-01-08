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
mod entities;
mod events;
mod game;
mod main_menu;
mod resources;
mod splash_screen;

pub use app_state::AppState;
pub use app_systems::despawn_screen;
use bevy_pkv::PkvStore;
use events::{MenuInteraction, PlayerAction};
use leafwing_input_manager::plugin::InputManagerPlugin;

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
        // Add all the general resources and their update systems (e.g. cursor position)
        .add_plugins(resources::ElementalistResourcesPlugin)
        // Add Camera
        .add_systems(
            Startup,
            (app_systems::setup_camera, app_systems::add_game_descriptor),
        )
        // Add plugins for splash screen and menu
        .add_plugins((splash_screen::SplashScreenPlugin, main_menu::MainMenuPlugin))
        // Add input processing
        .add_plugins((
            InputManagerPlugin::<PlayerAction>::default(),
            InputManagerPlugin::<MenuInteraction>::default(),
        ))
        // Spawn player (user controller)
        .add_systems(Startup, game::spawn_player_controller)
        // Handle player inputs
        .add_systems(
            Update,
            (
                game::player_control_system.after(resources::update_cursor_position_resource),
                game::menu_input,
            ),
        )
        // Sprite stuff
        .add_systems(OnEnter(AppState::InGame), game::setup_sprite)
        .add_systems(
            Update,
            (game::sprite_movement).run_if(in_state(AppState::InGame)),
        )
        // Launch
        .run();
}
