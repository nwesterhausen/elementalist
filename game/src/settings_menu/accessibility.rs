//! Has systems for the display settings menu.

use bevy::prelude::*;
use game_library::{font_resource::FontResource, settings::AccessibilitySettings};

use crate::common::colors;

use super::{
    base::{button_style, button_text},
    button_actions::ButtonAction,
};

/// Component for tagging entities that are part of the display settings menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(super) struct AccessibilitySettingsMenuEntity;

pub(super) fn show_accessibility_settings(
    mut commands: Commands,
    fonts: Res<FontResource>,
    accessibility_settings: Res<AccessibilitySettings>,
) {
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
            AccessibilitySettingsMenuEntity,
        ))
        .with_children(|parent| {
            // Game Title
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Accessibility Settings",
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
                    // font choice (as a row with a label and a button)
                    menu_buttons
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Start,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|row| {
                            // Button for rotating font family
                            row.spawn((button_style(), ButtonAction::RotateFontFamily))
                                .with_children(|button| {
                                    button.spawn(button_text(
                                        "Interface Font",
                                        fonts.interface_font.clone(),
                                    ));
                                });
                            // Text for current font family
                            row.spawn(button_text(
                                format!("{}", accessibility_settings.interface_font_family)
                                    .as_str(),
                                fonts.main_font.clone(),
                            ));
                        });
                    // Back button (=> settings)
                    menu_buttons
                        .spawn((button_style(), ButtonAction::BackToMenu))
                        .with_children(|button| {
                            button.spawn(button_text("Back", fonts.interface_font.clone()));
                        });
                });
        });
}
