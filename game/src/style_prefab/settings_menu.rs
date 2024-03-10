use bevy::prelude::*;
use game_library::colors;

/// Shared node style for the settings menus.
///
/// This node holds the menu buttons.
#[must_use]
pub fn settings_menu_column_node_bundle() -> NodeBundle {
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
pub fn settings_menu_full_node_bundle() -> NodeBundle {
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

/// Shared text bundle for settings menu titles.
///
/// This is the title of the menu.
#[must_use]
pub fn settings_menu_title_bundle(text: impl Into<String>, font: Handle<Font>) -> TextBundle {
    TextBundle {
        text: Text::from_section(
            text,
            TextStyle {
                font_size: 72.0,
                color: colors::TEXT_COLOR,
                font,
            },
        ),
        style: Style {
            align_self: AlignSelf::Center,
            ..default()
        },
        ..default()
    }
}

/// Shared row node style for the settings menus.
#[must_use]
pub fn settings_menu_button_row_node_bundle() -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Stretch,
            ..default()
        },
        ..default()
    }
}

/// Shared text bundle for settings menu "info" text. (The values of the settings)
#[must_use]
pub fn settings_menu_info_text_bundle(text: impl Into<String>, font: Handle<Font>) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font_size: 40.0,
            color: colors::TEXT_COLOR,
            font,
        },
    )
}
