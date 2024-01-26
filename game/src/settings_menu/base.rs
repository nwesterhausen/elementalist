//! Base systems for the menu.

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

use super::MenuState;

use crate::common::colors::{self, BACKGROUND_COLOR_50};

/// An entity tag for ease of cleanup when the menu is disabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct MenuEntity;

/// A tag specifically for the menu background.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct MenuBackground;

/// Clear the background (draw a blur) only when the menu is not disabled.
pub(super) fn clear_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    existing_background_query: Query<Entity, With<MenuBackground>>,
) {
    // Check if the background already exists.
    if let Ok(_) = existing_background_query.get_single() {
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
        MenuEntity,
    ));
}

/// System to setup settings menu.
///
/// When the settings menu is entered, we should setup the menu.
pub(super) fn transition_to_base_menu(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

/// Shared button style for the settings menus.
#[must_use]
pub(super) fn button_style() -> ButtonBundle {
    ButtonBundle {
        background_color: Color::NONE.into(),
        style: Style {
            margin: UiRect::px(10., 10., 0., 20.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Start,
            ..default()
        },
        ..default()
    }
}

/// Shared button style for the settings menus
#[must_use]
pub(super) fn button_text(text: &str, font: Handle<Font>) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font_size: 40.0,
            color: colors::TEXT_COLOR,
            font,
        },
    )
}

/// Shared node style for the settings menus.
///
/// This node holds the menu buttons.
#[must_use]
pub(super) fn node_style() -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Start,
            width: Val::Percent(50.0),
            margin: UiRect::px(20., 0., 40., 0.),
            ..default()
        },
        ..default()
    }
}

/// Shared node style for the settings menus.
///
/// This node holds everything (title and then button node and buttons)
#[must_use]
pub(super) fn super_node_style() -> NodeBundle {
    NodeBundle {
        style: Style {
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.0)),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        ..default()
    }
}
