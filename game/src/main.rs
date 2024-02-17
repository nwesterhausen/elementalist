//! # Elementalist
//!
//! A rogue-lite game where you play as an elementalist.

// Hide the console window on Windows when not in debug mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{
    asset::AssetMetaCheck,
    log::LogPlugin,
    prelude::*,
    render::{render_resource::WgpuFeatures, settings::WgpuSettings, RenderPlugin},
};
use bevy_rapier2d::prelude::*;
use game_library::{
    data_loader::storage::GameData, state::Game, GeneratedMaps, Layer, LayerPlugin,
    MarkersToBiomes, NoisePlugin, PhysicsPlugin, SchedulingPlugin,
};
use in_game::InGamePlugin;
use leafwing_input_manager::plugin::InputManagerPlugin;

mod app_info;
mod app_systems;
mod camera;
#[cfg(debug_assertions)]
mod dev_systems;
mod events;
mod game_overlays;
mod in_game;
mod main_menu;
mod player;
mod resources;
mod settings_menu;
mod spells;
mod splash_screen;

pub use app_systems::despawn_with_tag;
use events::{MenuInteraction, PlayerAction};

fn main() {
    // Set the wgpu settings per bevy_hanabi
    let mut wgpu_settings = WgpuSettings::default();
    wgpu_settings
        .features
        .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);

    // Set the log level based on whether we are in debug mode or not
    let (log_level, log_filter) = if cfg!(debug_assertions) {
        // If in debug mode, set the log level to debug
        (
            bevy::log::Level::INFO,
            "wgpu=warn,bevy_hanabi=warn,elementalist=info",
        )
    } else {
        // If in release mode, set the log level to warn
        (bevy::log::Level::WARN, "elementalist=info")
    };

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
                .set(ImagePlugin::default_nearest())
                // Configure the log plugin
                .set(LogPlugin {
                    level: log_level,
                    filter: log_filter.to_string(),
                })
                // Add our the wgpu settings per bevy_hanabi
                .set(RenderPlugin {
                    render_creation: wgpu_settings.into(),
                }),
        )
        // Add the gameplay plugins
        .add_plugins(ElementalistGameplayPlugins)
        // Add the debug plugin if in debug mode (this adds the inspector)
        .add_plugins(ElementalistDebugPlugin)
        // Add all the general resources and their update systems (e.g. cursor position)
        .add_plugins(resources::ElementalistResourcesPlugin)
        // Add plugins for Settings and the menus
        .add_plugins((
            settings_menu::SettingsMenuPlugin,
            splash_screen::SplashScreenPlugin,
            main_menu::MainMenuPlugin,
            game_overlays::GameOverlaysPlugin,
        ))
        .add_plugins(NoisePlugin)
        // Some test plugins for environment stuff
        .add_systems(OnEnter(Game::Playing), spawn_random_environment)
        .add_systems(OnExit(Game::Playing), despawn_with_tag::<EnvironmentStuff>)
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
        app.insert_resource(ClearColor(game_library::colors::CLEAR_COLOR));
        // Add the window icon
        app.add_systems(Startup, app_systems::set_window_icon);
        // Add camera plugin
        app.add_plugins(camera::CameraPlugin);
        // Add the schedule plugin
        app.add_plugins(SchedulingPlugin);
        // Add the physics plugin
        app.add_plugins(PhysicsPlugin);
        // Add the layer plugin
        app.add_plugins(LayerPlugin);
    }
}

/// Debug plugin loader.
///
/// This adds the debug plugin _only_ when in debug mode.
struct ElementalistDebugPlugin;

impl Plugin for ElementalistDebugPlugin {
    // Unused variables allowed because when in release mode, `app` will not be used.
    #[allow(unused_variables)]
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
        app.add_plugins(InGamePlugin);
    }
}

/// Spawn some trees as a test
fn spawn_random_environment(
    mut commands: Commands,
    game_data: Res<GameData>,
    generated_map: Res<GeneratedMaps>,
) {
    // get the biomes for the current map
    let Some(realm) = game_data.realms.get("simple test realm") else {
        tracing::error!("No realm found for 'simple test realm'");
        return;
    };

    // spawn ground over whole map
    let biome_map = realm.markers_to_biomes(generated_map.biome_map.clone().as_slice());
    for (i, row) in biome_map.iter().enumerate() {
        for (j, biome) in row.iter().enumerate() {
            // Convert the map coordinates to world coordinates
            let ground_translation = generated_map.map_to_world((i, j));

            // Get random info from the biome
            let Some(rnd_ground) = biome.random_ground_tile() else {
                tracing::warn!("No ground tile found for biome at ({}, {})", i, j);
                continue;
            };

            let Some(ground) = game_data.tile_atlas.get(rnd_ground.0) else {
                tracing::error!("Failed to load ground tileselt: {}", rnd_ground.0);
                continue;
            };
            let ground_id = rnd_ground.1;

            let ground_transform = Transform::from_translation(ground_translation);
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: ground.clone(),
                    sprite: bevy::sprite::TextureAtlasSprite::new(ground_id),
                    transform: ground_transform,
                    ..Default::default()
                },
                EnvironmentStuff,
                Layer::Background(i16::MAX),
            ));
        }
    }

    // spawn trees and rocks

    let Some(tree) = game_data.tile_atlas.get("trees") else {
        tracing::error!("Failed to load tree tile");
        return;
    };
    let Some(rock) = game_data.tile_atlas.get("rock") else {
        tracing::error!("Failed to load rock tile");
        return;
    };

    for (i, row) in generated_map.object_map.iter().enumerate() {
        for (j, object_id) in row.iter().enumerate() {
            let obj = match object_id {
                4 => Some((tree, 0)),
                13 => Some((tree, 1)),
                19 => Some((tree, 2)),
                3 | 15 => Some((rock, 0)),
                6 | 17 => Some((rock, 1)),
                _ => None,
            };
            if let Some(obj) = obj {
                let mut obj_transform =
                    Transform::from_translation(generated_map.map_to_world((i, j)));

                // create depth from f32 to i16 with the f32 range mapping to i16 range, since translation.y has valid
                // values between -0.5 * generated_map.dimensions().1 and 0.5 * generated_map.dimensions().1
                let depth = (obj_transform.translation.y
                    / (0.5 * generated_map.dimensions().1 as f32)
                    * f32::from(i16::MAX)) as i16;

                obj_transform.scale = Vec3::splat(1.0);
                commands.spawn((
                    SpriteSheetBundle {
                        texture_atlas: obj.0.clone(),
                        sprite: bevy::sprite::TextureAtlasSprite::new(obj.1),
                        transform: obj_transform,
                        ..Default::default()
                    },
                    EnvironmentStuff,
                    RigidBody::Fixed,
                    // todo: make this reference the actual size of the sprite
                    Collider::cuboid(6.0, 4.0),
                    Layer::Foreground(depth),
                ));
            }
        }
    }
}

/// Test environment stuff
#[derive(Component)]
pub struct EnvironmentStuff;
