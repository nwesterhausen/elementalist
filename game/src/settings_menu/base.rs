//! Base systems for the menu.

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

use crate::resources::{colors::BACKGROUND_COLOR_50, MenuState};

/// An entity tag for ease of cleanup when the menu is disabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct SettingsMenuEntity;

/// A tag specifically for the menu background.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct SettingsMenuBackground;

/// Clear the background (draw a blur) only when the menu is not disabled.
pub(super) fn clear_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    existing_background_query: Query<Entity, With<SettingsMenuBackground>>,
) {
    // Check if the background already exists.
    if existing_background_query.get_single().is_ok() {
        // If it does, just return.
        return;
    }

    let Ok(window) = window_query.get_single() else {
        tracing::warn!("Failed to get window size for menu background");
        return;
    };

    // Quad that draws over the whole screen.
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(window.width(), window.height())).into())
                .into(),
            material: materials.add(ColorMaterial::from(BACKGROUND_COLOR_50)),
            transform: Transform::from_xyz(0., 0., 10.),
            ..default()
        },
        SettingsMenuBackground,
    ));
}

/// System to setup settings menu.
///
/// When the settings menu is entered, we should setup the menu.
pub(super) fn transition_to_base_menu(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}
