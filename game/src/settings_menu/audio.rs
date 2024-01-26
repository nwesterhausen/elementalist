//! Has systems for the display settings menu.

use bevy::prelude::*;
use game_library::{
    font_resource::FontResource, menu_helper::make_text_bundle, settings::VolumeSettings,
};

use crate::common::colors;

use super::{
    base::{button_style, button_text, node_style, super_node_style},
    button_actions::ButtonAction,
};

/// Component for tagging entities that are part of the display settings menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(super) struct AudioSettingsMenuEntity;

pub(super) fn show_audio_settings(
    mut commands: Commands,
    fonts: Res<FontResource>,
    volume_settings: Res<VolumeSettings>,
) {
    commands
        .spawn((super_node_style(), AudioSettingsMenuEntity))
        .with_children(|parent| {
            // Menu Title
            parent.spawn(make_text_bundle(
                "Audio Settings",
                fonts.display_font.clone(),
                72.0,
                colors::TEXT_COLOR,
                AlignSelf::Center,
            ));
            // #### MENU BUTTONS #####
            parent.spawn(node_style()).with_children(|menu_buttons| {
                // Volume slider for main volume
                // For display purposes, use a node here as a row to show the button ("Main Volume") and then the value
                // of main volume in a text node.
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
                        // Button for main volume
                        row.spawn((button_style(), ButtonAction::SettingsAudio))
                            .with_children(|button| {
                                button.spawn(button_text("Main", fonts.display_font.clone()));
                            });
                        // Text for main volume
                        row.spawn(TextBundle::from_section(
                            format!("{:.2}", volume_settings.main),
                            TextStyle {
                                font_size: 40.0,
                                color: colors::TEXT_COLOR,
                                font: fonts.interface_font.clone(),
                            },
                        ));
                    });
                // Volume slider for music volume
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
                        // Button for music volume
                        row.spawn((button_style(), ButtonAction::SettingsAudio))
                            .with_children(|button| {
                                button.spawn(button_text("Music", fonts.display_font.clone()));
                            });
                        // Text for music volume
                        row.spawn(TextBundle::from_section(
                            format!("{:.2}", volume_settings.music),
                            TextStyle {
                                font_size: 40.0,
                                color: colors::TEXT_COLOR,
                                font: fonts.interface_font.clone(),
                            },
                        ));
                    });
                // Volume slider for sound effects volume
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
                        // Button for sound effects volume
                        row.spawn((button_style(), ButtonAction::SettingsAudio))
                            .with_children(|button| {
                                button.spawn(button_text(
                                    "Sound Effects",
                                    fonts.display_font.clone(),
                                ));
                            });
                        // Text for sound effects volume
                        row.spawn(TextBundle::from_section(
                            format!("{:.2}", volume_settings.sfx),
                            TextStyle {
                                font_size: 40.0,
                                color: colors::TEXT_COLOR,
                                font: fonts.interface_font.clone(),
                            },
                        ));
                    });
                // Back button (=> settings)
                menu_buttons
                    .spawn((button_style(), ButtonAction::BackToMenu))
                    .with_children(|button| {
                        button.spawn(button_text("Back", fonts.display_font.clone()));
                    });
            });
        });
}
