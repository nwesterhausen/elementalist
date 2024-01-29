//! # Elementalist
//!
//! A rogue-lite game where you play as an elementalist.

// Hide the console window on Windows when not in debug mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{asset::AssetMetaCheck, prelude::*};
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
        // Add the gameplay plugins
        .add_plugins(ElementalistGameplayPlugins)
        // Add the debug plugin if in debug mode (this adds the inspector)
        .add_plugins(ElementalistDebugPlugin)
        // Add all the general resources and their update systems (e.g. cursor position)
        .add_plugins(resources::ElementalistResourcesPlugin)
        // Add plugins for Settings and the menus
        .add_plugins((
            // Add the menu plugin
            settings_menu::SettingsMenuPlugin,
            // Add our plugins for the menu screen and the splash screen
            splash_screen::SplashScreenPlugin,
            main_menu::MainMenuPlugin,
        ))
        // Launch
        .run();
}

/// Elementalist defaults for the "insert this resource" type of thing.
///
/// This adds resources specific for [`DefaultPlugins`]
struct ElementalistDefaultPlugins;

impl Plugin for ElementalistDefaultPlugins {
    fn build(&self, app: &mut App) {
        // Never attempts to look up meta files. The default meta configuration will be used for each asset.
        app.insert_resource(AssetMetaCheck::Never);
        // The clear color is the color the screen is cleared to before each frame is drawn
        app.insert_resource(ClearColor(resources::colors::CLEAR_COLOR));
        // Add the window icon
        app.add_systems(Startup, app_systems::set_window_icon);
        // Add camera plugin
        app.add_plugins(camera::CameraPlugin);
    }
}

/// Debug plugin loader.
///
/// This adds the debug plugin _only_ when in debug mode.
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

/// Gameplay plugins.
///
/// These add the gameplay functionality to the game.
struct ElementalistGameplayPlugins;

impl Plugin for ElementalistGameplayPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // Add the plugin for the player
            player::PlayerPlugin,
            // Add the plugin for the spells
            spells::SpellsPlugin,
            // Add the plugin for the movement
            resources::movement::MovementPlugin,
            // Input processing
            InputManagerPlugin::<PlayerAction>::default(),
            InputManagerPlugin::<MenuInteraction>::default(),
        ));
    }
}
