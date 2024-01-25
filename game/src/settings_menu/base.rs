//! Base systems for the menu.

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

use super::state::MenuState;

/// An entity tag for ease of cleanup when the menu is disabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct MenuEntity;

/// Clear the background (draw a blur) only when the menu is not disabled.
pub(super) fn clear_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window_query.get_single() else {
        tracing::warn!("Failed to get window size for menu background");
        return;
    };
    // Quad
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(window.width(), window.height())).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        MenuEntity,
    ));
}

/// Cleanup the menu entities when the menu is disabled.
pub(super) fn cleanup_menu_entities(
    mut commands: Commands,
    menu_query: Query<Entity, With<MenuEntity>>,
) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
