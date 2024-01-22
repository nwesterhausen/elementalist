//! # Elementalist
//!
//! A rogue-lite game where you play as an elementalist.

// Hide the console window on Windows when not in debug mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{asset::AssetMetaCheck, prelude::*};
use leafwing_input_manager::plugin::InputManagerPlugin;

mod app_info;
mod app_state;
mod app_systems;
mod camera;
mod common;
#[cfg(debug_assertions)]
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
        // Add our custom default plugins
        .add_plugins(ElementalistDefaultPlugins)
        // Add the default plugins
        .add_plugins(
            DefaultPlugins
                // Set basic window properties
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: app_info::game_name_and_version(),
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

/// Elementalist defaults for the "insert this resource" type of thing.
///
/// This also handles adding the debug plugin _only_ when in debug mode.
struct ElementalistDefaultPlugins;

impl Plugin for ElementalistDefaultPlugins {
    fn build(&self, app: &mut App) {
        // Never attempts to look up meta files. The default meta configuration will be used for each asset.
        app.insert_resource(AssetMetaCheck::Never);
        // The clear color is the color the screen is cleared to before each frame is drawn
        app.insert_resource(ClearColor(Color::DARK_GREEN));
        // Add the window icon
        app.add_systems(Startup, app_systems::set_window_icon);
        // When in debug mode, add the debug plugin.
        #[cfg(debug_assertions)]
        {
            app.add_plugins(dev_systems::DevSystemsPlugin);
        }
    }
}
