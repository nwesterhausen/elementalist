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
use game_library::{colors, data_loader::storage::GameData, state::AppState};
use leafwing_input_manager::plugin::InputManagerPlugin;
use rand::Rng;

mod app_info;
mod app_systems;
mod camera;
#[cfg(debug_assertions)]
mod dev_systems;
mod events;
mod game_overlays;
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
        .add_systems(OnEnter(AppState::InGame), spawn_random_environment)
        .add_systems(
            OnExit(AppState::InGame),
            despawn_with_tag::<EnvironmentStuff>,
        )
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
    }
}

/// Spawn some trees as a test
fn spawn_random_environment(
    mut commands: Commands,
    game_data: Res<GameData>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let Some(tree) = game_data.tile_atlas.get("trees") else {
        tracing::error!("Failed to load tree tile");
        return;
    };
    let Some(rock) = game_data.tile_atlas.get("rock") else {
        tracing::error!("Failed to load rock tile");
        return;
    };

    let rng = &mut rand::thread_rng();

    // spawn a solid background color for the grass
    let grass_material = materials.add(colors::THUNDER.into());
    let big_rectangle = meshes.add(Mesh::from(shape::Quad {
        size: Vec2::new(1000.0, 1000.0),
        flip: false,
    }));
    // spawn it in the center of the screen
    commands.spawn((
        ColorMesh2dBundle {
            material: grass_material,
            mesh: big_rectangle.into(),
            transform: Transform::from_xyz(0.0, 0.0, -10.0),
            ..Default::default()
        },
        EnvironmentStuff,
    ));

    let mut no_tree = 0.0;

    for i in -16..16 {
        for j in -16..16 {
            if rng.gen_bool(0.05 + no_tree) {
                let index = rng.gen_range(0..=2);
                let jitter = rng.gen_range(-16.0..16.0);
                #[allow(clippy::cast_precision_loss)]
                let y_val = (j as f32).mul_add(32.0, jitter);
                commands.spawn((
                    SpriteSheetBundle {
                        texture_atlas: tree.clone(),
                        sprite: bevy::sprite::TextureAtlasSprite::new(index),
                        transform: Transform {
                            #[allow(clippy::cast_precision_loss)]
                            translation: Vec3::new(
                                (i as f32).mul_add(32.0, jitter),
                                y_val,
                                y_val / -100.0,
                            ),
                            scale: Vec3::splat(1.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    EnvironmentStuff,
                ));
                no_tree = 0.0;
            } else if rng.gen_bool(0.1) {
                let index = rng.gen_range(0..=2);
                let jitter = if index == 0 {
                    0.
                } else {
                    rng.gen_range(-16.0..16.0)
                };
                #[allow(clippy::cast_precision_loss)]
                let y_val = (j as f32).mul_add(32.0, jitter);
                commands.spawn((
                    SpriteSheetBundle {
                        texture_atlas: rock.clone(),
                        sprite: bevy::sprite::TextureAtlasSprite::new(index),
                        transform: Transform {
                            #[allow(clippy::cast_precision_loss)]
                            translation: Vec3::new(
                                (i as f32).mul_add(32.0, jitter),
                                y_val,
                                y_val / -100.0,
                            ),
                            scale: Vec3::splat(1.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    EnvironmentStuff,
                ));
            }
            no_tree += 0.01;
        }
    }
}

/// Test environment stuff
#[derive(Component)]
pub struct EnvironmentStuff;
