//! Has systems for the display settings menu.

use bevy::prelude::*;
use game_library::{
    font_resource::FontResource,
    menu_helper::make_text_bundle,
    settings::{SettingCategory, SettingChanged, VolumeSettings},
};

use crate::common::colors;

use super::{
    base::{button_style, button_text, node_style, super_node_style},
    button_actions::ButtonAction,
    events::{ChangeSetting, IndividualSetting},
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
                        row.spawn((button_style(), ButtonAction::IncrementMainVolume))
                            .with_children(|button| {
                                button.spawn(button_text("Main", fonts.interface_font.clone()));
                            });
                        // Text for main volume
                        row.spawn(TextBundle::from_section(
                            format!("{:.2}", volume_settings.main),
                            TextStyle {
                                font_size: 40.0,
                                color: colors::TEXT_COLOR,
                                font: fonts.main_font.clone(),
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
                        row.spawn((button_style(), ButtonAction::IncrementMusicVolume))
                            .with_children(|button| {
                                button.spawn(button_text("Music", fonts.interface_font.clone()));
                            });
                        // Text for music volume
                        row.spawn(TextBundle::from_section(
                            format!("{:.2}", volume_settings.music),
                            TextStyle {
                                font_size: 40.0,
                                color: colors::TEXT_COLOR,
                                font: fonts.main_font.clone(),
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
                        row.spawn((button_style(), ButtonAction::IncrementSoundEffectsVolume))
                            .with_children(|button| {
                                button.spawn(button_text(
                                    "Sound Effects",
                                    fonts.interface_font.clone(),
                                ));
                            });
                        // Text for sound effects volume
                        row.spawn(TextBundle::from_section(
                            format!("{:.2}", volume_settings.sfx),
                            TextStyle {
                                font_size: 40.0,
                                color: colors::TEXT_COLOR,
                                font: fonts.main_font.clone(),
                            },
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

/// System to handle the audio menu button actions.
pub(super) fn handle_audio_setting_changes(
    mut er_change_setting: EventReader<ChangeSetting>,
    mut gameplay_settings: ResMut<VolumeSettings>,
    mut ew_setting_changed: EventWriter<SettingChanged>,
) {
    for change_setting in er_change_setting.read() {
        match change_setting.setting {
            IndividualSetting::MainVolume => {
                gameplay_settings.main.increment();
                ew_setting_changed.send(SettingChanged(SettingCategory::Volume));
            }
            IndividualSetting::MusicVolume => {
                gameplay_settings.music.increment();
                ew_setting_changed.send(SettingChanged(SettingCategory::Volume));
            }
            IndividualSetting::SoundEffectsVolume => {
                gameplay_settings.sfx.increment();
                ew_setting_changed.send(SettingChanged(SettingCategory::Volume));
            }
            _ => {}
        }
    }
}
