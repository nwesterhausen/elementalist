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
    utils::HashMap,
};
use game_library::{
    data_loader::storage::GameData, enums::biome::Biome, state::Game, GeneratedMaps,
    MarkersToBiomes, NoisePlugin, SchedulingPlugin,
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
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    game_data: Res<GameData>,
    generated_map: Res<GeneratedMaps>,
) {
    // get the biomes for the current map
    let Some(realm) = game_data.realms.get("simple test realm") else {
        tracing::error!("No realm found for 'simple test realm'");
        return;
    };

    let biomes = &realm.markers_to_biomes(generated_map.biome_map.as_slice());
    // from the biomes which is vec of vec of Biome, turn that into a flattened, deduped
    // vec of Biome, and then map with `get_color` to get the colors we will use.
    let mut color_materials: HashMap<Biome, Handle<ColorMaterial>> = HashMap::new();
    for biome_row in biomes {
        for biome in biome_row {
            let color = biome.get_color();
            let material = materials.add(color.into());
            color_materials.insert(*biome, material.clone());
        }
    }

    // add the 16x16 quad mesh
    let mesh = meshes.add(Mesh::from(shape::Quad {
        size: Vec2::new(16.0, 16.0),
        flip: false,
    }));

    // spawn a colored 16x16 quad for each tile in the biome map.
    let mesh_z = -100.0;
    for (i, row) in biomes.iter().enumerate() {
        for (j, biome) in row.iter().enumerate() {
            let Some(material) = color_materials.get(biome) else {
                tracing::error!("No material found for biome: {:?}", biome);
                continue;
            };
            let mut mesh_transform =
                Transform::from_translation(generated_map.map_to_world((i, j)));
            mesh_transform.translation.z = mesh_z;
            // use the `map_to_world` function to convert the biome map coordinates to world coordinates.
            commands.spawn((
                ColorMesh2dBundle {
                    material: material.clone(),
                    transform: mesh_transform,
                    mesh: mesh.clone().into(),
                    ..Default::default()
                },
                EnvironmentStuff,
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
                4 | 8 => Some((tree, 0)),
                5 | 13 => Some((tree, 1)),
                0 | 19 => Some((tree, 2)),
                3 | 7 | 15 => Some((rock, 0)),
                6 | 12 | 14 => Some((rock, 1)),
                _ => None,
            };
            if let Some(obj) = obj {
                let mut obj_transform =
                    Transform::from_translation(generated_map.map_to_world((i, j)));
                #[allow(clippy::cast_precision_loss)]
                let z_val = obj_transform.translation.y / -10.0;
                obj_transform.translation.z = z_val;
                obj_transform.scale = Vec3::splat(1.0);
                commands.spawn((
                    SpriteSheetBundle {
                        texture_atlas: obj.0.clone(),
                        sprite: bevy::sprite::TextureAtlasSprite::new(obj.1),
                        transform: obj_transform,
                        ..Default::default()
                    },
                    EnvironmentStuff,
                ));
            }
        }
    }
}

/// Test environment stuff
#[derive(Component)]
pub struct EnvironmentStuff;
