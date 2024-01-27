//! Has systems for the display settings menu.

use bevy::prelude::*;
use game_library::{
    font_resource::FontResource,
    settings::{SettingCategory, SettingChanged, VolumeSettings},
};

use crate::common::buttons::style_prefab;

use super::{
    base::MenuEntity,
    button_actions::{ButtonAction, SettingsMenuButton},
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
        .spawn((
            style_prefab::settings_menu_full_node_bundle(),
            AudioSettingsMenuEntity,
            MenuEntity,
        ))
        .with_children(|parent| {
            // Menu Title
            parent.spawn(style_prefab::settings_menu_title_bundle(
                "Audio",
                fonts.display_font.clone(),
            ));
            // #### MENU BUTTONS #####
            parent
                .spawn(style_prefab::settings_menu_column_node_bundle())
                .with_children(|menu_buttons| {
                    // Volume slider for main volume
                    // For display purposes, use a node here as a row to show the button ("Main Volume") and then the value
                    // of main volume in a text node.
                    menu_buttons
                        .spawn(style_prefab::settings_menu_button_row_node_bundle())
                        .with_children(|row| {
                            // Button for main volume
                            row.spawn((
                                style_prefab::menu_button_bundle(),
                                ButtonAction::IncrementMainVolume,
                                SettingsMenuButton,
                            ))
                            .with_children(|button| {
                                button.spawn(style_prefab::menu_button_text(
                                    "Main",
                                    fonts.interface_font.clone(),
                                ));
                            });
                            // Text for main volume
                            row.spawn(style_prefab::settings_menu_info_text_bundle(
                                format!("{:.2}", volume_settings.main),
                                fonts.main_font.clone(),
                            ));
                        });
                    // Volume slider for music volume
                    menu_buttons
                        .spawn(style_prefab::settings_menu_button_row_node_bundle())
                        .with_children(|row| {
                            // Button for music volume
                            row.spawn((
                                style_prefab::menu_button_bundle(),
                                ButtonAction::IncrementMusicVolume,
                                SettingsMenuButton,
                            ))
                            .with_children(|button| {
                                button.spawn(style_prefab::menu_button_text(
                                    "Music",
                                    fonts.interface_font.clone(),
                                ));
                            });
                            // Text for music volume
                            row.spawn(style_prefab::settings_menu_info_text_bundle(
                                format!("{:.2}", volume_settings.music),
                                fonts.main_font.clone(),
                            ));
                        });
                    // Volume slider for sound effects volume
                    menu_buttons
                        .spawn(style_prefab::settings_menu_button_row_node_bundle())
                        .with_children(|row| {
                            // Button for sound effects volume
                            row.spawn((
                                style_prefab::menu_button_bundle(),
                                ButtonAction::IncrementSoundEffectsVolume,
                                SettingsMenuButton,
                            ))
                            .with_children(|button| {
                                button.spawn(style_prefab::menu_button_text(
                                    "Sound Effects",
                                    fonts.interface_font.clone(),
                                ));
                            });
                            // Text for sound effects volume
                            row.spawn(style_prefab::settings_menu_info_text_bundle(
                                format!("{:.2}", volume_settings.sfx),
                                fonts.main_font.clone(),
                            ));
                        });
                    // Back button (=> settings)
                    menu_buttons
                        .spawn((
                            style_prefab::menu_button_bundle(),
                            ButtonAction::BackToMenu,
                            SettingsMenuButton,
                        ))
                        .with_children(|button| {
                            button.spawn(style_prefab::menu_button_text(
                                "Back",
                                fonts.interface_font.clone(),
                            ));
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
