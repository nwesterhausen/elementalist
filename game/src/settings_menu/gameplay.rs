//! Has systems for the display settings menu.

use bevy::prelude::*;
use game_library::{font_resource::FontResource, settings::GameplaySettings};

use crate::common::colors;

use super::button_actions::ButtonAction;

/// Component for tagging entities that are part of the display settings menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(super) struct GameplaySettingsMenuEntity;

pub(super) fn show_gameplay_settings(
    mut commands: Commands,
    fonts: Res<FontResource>,
    mut gameplay_settings: ResMut<GameplaySettings>,
) {
    // Common style for all buttons on the screen
    let button_style = Style {
        margin: UiRect::px(10., 10., 0., 20.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Start,
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: colors::TEXT_COLOR,
        font: fonts.display_font.clone(),
    };

    commands
        .spawn((
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
            },
            GameplaySettingsMenuEntity,
        ))
        .with_children(|parent| {
            // Game Title
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Gameplay Settings",
                    TextStyle {
                        font: fonts.display_font.clone(),
                        font_size: 72.0,
                        color: colors::TEXT_COLOR,
                    },
                ),
                style: Style {
                    align_self: AlignSelf::Center,
                    ..default()
                },
                ..default()
            });
            // #### MENU BUTTONS #####
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Start,
                        width: Val::Percent(100.0),
                        margin: UiRect::px(20., 0., 40., 0.),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|menu_buttons| {
                    // Back button (=> settings)
                    menu_buttons
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: Color::NONE.into(),
                                ..default()
                            },
                            ButtonAction::BackToMenu,
                        ))
                        .with_children(|button| {
                            button
                                .spawn(TextBundle::from_section("Back", button_text_style.clone()));
                        });
                });
        });
}
