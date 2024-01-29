use bevy::prelude::*;

use super::colors;

/// Tag component used to mark which setting is currently selected
#[derive(Component)]
pub struct SelectedOption;

/// System for changing button colors when hovered, etc
///
/// * `interaction_query`: grabs all the buttons that have been interacted with, with the components
///   Interaction, children, and if they are a selected option (e.g. part of a radio group). It grabs
///   what has changed about the interaction (i.e. if it has changed at all)
/// * `text_query`: let's us grab the text component of the button
#[allow(clippy::type_complexity)]
pub fn interaction_system(
    mut interaction_query: Query<
        (&Interaction, &Children, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    // Loop through all the buttons that have been interacted with (and all the components we grabbed)
    for (interaction, children, selected) in &mut interaction_query {
        // Try to grab the text component of the button, and if we can't we will just skip (continue)
        // We are changing the text color and not the button color because drawing big squares is dumb. :)
        let Ok(mut text) = text_query.get_mut(children[0]) else {
            tracing::error!("Button has no text (and was probably supposed to)");
            continue;
        };
        // Match the interaction and selected option to change the text color
        // i.e., hover, not-hover & selected, pressed, and not-pressed & not selected. (or close that that)
        match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => {
                text.sections[0].style.color = colors::SELECTED_TEXT_COLOR;
            }
            (Interaction::Hovered, Some(_)) => {
                text.sections[0].style.color = colors::HOVERED_TEXT_COLOR_ALTERNATE;
            }
            (Interaction::Hovered, None) => {
                text.sections[0].style.color = colors::HOVERED_TEXT_COLOR;
            }
            (Interaction::None, None) => text.sections[0].style.color = colors::TEXT_COLOR,
        }
    }
}

pub mod style_prefab {
    use crate::resources::colors;
    use bevy::prelude::*;

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
    pub fn settings_menu_info_text_bundle(
        text: impl Into<String>,
        font: Handle<Font>,
    ) -> TextBundle {
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
}
