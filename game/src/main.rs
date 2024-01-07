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

use app_systems::spawn_player;
use bevy::prelude::*;

mod app_info;
mod app_state;
mod app_systems;
mod entities;
mod events;
mod game;
mod main_menu;
mod splash_screen;

pub use app_state::AppState;
pub use app_systems::despawn_screen;
use bevy_pkv::PkvStore;
use entities::Player;
use events::{player_aim_at_cursor, MenuInteraction, PlayerAction};
use leafwing_input_manager::{action_state::ActionState, plugin::InputManagerPlugin};

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
        .add_systems(Startup, spawn_player)
        // Handle player inputs
        .add_systems(
            Update,
            (
                player_aim_at_cursor,
                handle_input_test.after(player_aim_at_cursor),
            ),
        )
        // Launch
        .run();
}

fn handle_input_test(query: Query<&ActionState<PlayerAction>, With<Player>>) {
    let action_state = query.single();
    if action_state.pressed(PlayerAction::Move) {
        if let Some(axis_pair) = action_state.clamped_axis_pair(PlayerAction::Move) {
            println!("Move: {:?}", axis_pair);
        } else {
            println!("Move");
        }
    }
    if action_state.pressed(PlayerAction::Look) {
        if let Some(axis_pair) = action_state.clamped_axis_pair(PlayerAction::Look) {
            println!("Look: {:?}", axis_pair);
        } else {
            println!("Look");
        }
    }
    if action_state.just_pressed(PlayerAction::CastPrimary) {
        println!("CastPrimary");
    }
    if action_state.just_pressed(PlayerAction::CastSecondary) {
        println!("CastSecondary");
    }
    if action_state.just_pressed(PlayerAction::CastDefensive) {
        println!("CastDefensive");
    }
    if action_state.just_pressed(PlayerAction::CastUltimate) {
        println!("CastUltimate");
    }
    if action_state.just_pressed(PlayerAction::ToggleAutoCast) {
        println!("ToggleAutoCast");
    }
    if action_state.just_pressed(PlayerAction::ToggleAutoAim) {
        println!("ToggleAutoAim");
    }
    if action_state.just_pressed(PlayerAction::Interact) {
        println!("Interact");
    }
}
