//! # Elementalist
//!
//! A rogue-lite game where you play as an elementalist.

// Hide the console window on Windows when not in debug mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{asset::AssetMetaCheck, prelude::*};
use game_library::settings::SettingsPlugin;
use leafwing_input_manager::plugin::InputManagerPlugin;

mod app_info;
mod app_systems;
mod camera;
#[cfg(debug_assertions)]
mod dev_systems;
mod events;
mod main_menu;
mod player;
mod resources;
mod settings_menu;
mod spells;
mod splash_screen;

pub use app_systems::despawn_with_tag;
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
                // Nearest neighbor scaling (pixel art)
                .set(ImagePlugin::default_nearest()),
        )
        // Add the debug plugin if in debug mode
        .add_plugins(ElementalistDebugPlugin)
        // Add all the general resources and their update systems (e.g. cursor position)
        .add_plugins(resources::ElementalistResourcesPlugin)
        // Add input processing
        .add_plugins((
            InputManagerPlugin::<PlayerAction>::default(),
            InputManagerPlugin::<MenuInteraction>::default(),
        ))
        // Add camera plugin
        .add_plugins(camera::CameraPlugin)
        // Add our plugins for the menu screen and the splash screen
        .add_plugins((splash_screen::SplashScreenPlugin, main_menu::MainMenuPlugin))
        // Add the player plugin
        .add_plugins(player::PlayerPlugin)
        // Add the spells plugin
        .add_plugins(spells::SpellsPlugin)
        // Add the movement plugin
        .add_plugins(resources::movement::MovementPlugin)
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
        app.insert_resource(ClearColor(resources::colors::CLEAR_COLOR));
        // Add the window icon
        app.add_systems(Startup, app_systems::set_window_icon);
        // Add the settings plugin
        app.add_plugins(SettingsPlugin);
        // Add the menu plugin
        app.add_plugins(settings_menu::SettingsMenuPlugin);
    }
}

/// Debug plugin loader.
struct ElementalistDebugPlugin;

impl Plugin for ElementalistDebugPlugin {
    fn build(&self, app: &mut App) {
        // When in debug mode, add the debug plugin.
        #[cfg(debug_assertions)]
        {
            app.add_plugins(dev_systems::DevSystemsPlugin);
        }
    }
}
