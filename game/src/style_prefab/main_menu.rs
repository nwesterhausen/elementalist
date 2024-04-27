use bevy::prelude::*;
use elementalist_game_library::colors;

/// Shared button style for the settings menus.
#[must_use]
pub fn menu_button_bundle() -> ButtonBundle {
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
pub fn menu_button_text(text: impl Into<String>, font: Handle<Font>) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font_size: 40.0,
            color: colors::TEXT_COLOR,
            font,
        },
    )
}

/// Node style for the main menu column.
#[must_use]
pub fn main_menu_column_node_bundle() -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Start,
            width: Val::Percent(100.0),
            margin: UiRect::px(20., 0., 40., 0.),
            ..default()
        },
        ..default()
    }
}

/// Parent node style for the main menu.
#[must_use]
pub fn main_menu_full_node_bundle() -> NodeBundle {
    NodeBundle {
        style: Style {
            align_items: AlignItems::Start,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.0)),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        ..default()
    }
}

/// Game title text bundle for the main menu.
#[must_use]
pub fn main_menu_title_bundle(text: impl Into<String>, font: Handle<Font>) -> TextBundle {
    TextBundle {
        text: Text::from_section(
            text,
            TextStyle {
                font,
                font_size: 112.0,
                color: colors::TEXT_COLOR,
            },
        ),
        style: Style {
            align_self: AlignSelf::Center,
            ..default()
        },
        ..default()
    }
}
