//! Has systems for the display settings menu.

use bevy::prelude::*;
use game_library::{font_resource::FontResource, settings::VideoSettings};

use crate::common::buttons::style_prefab;

use super::{
    base::MenuEntity,
    button_actions::{ButtonAction, SettingsMenuButton},
};

/// Component for tagging entities that are part of the display settings menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(super) struct DisplaySettingsMenuEntity;

pub(super) fn show_display_settings(
    mut commands: Commands,
    fonts: Res<FontResource>,
    video_settings: Res<VideoSettings>,
) {
    commands
        .spawn((
            style_prefab::settings_menu_full_node_bundle(),
            DisplaySettingsMenuEntity,
            MenuEntity,
        ))
        .with_children(|parent| {
            // Game Title
            parent.spawn(style_prefab::settings_menu_title_bundle(
                "Display Settings",
                fonts.display_font.clone(),
            ));
            // #### MENU BUTTONS #####
            parent
                .spawn(style_prefab::settings_menu_column_node_bundle())
                .with_children(|menu_buttons| {
                    // Button and label for the video settings property display_scale
                    menu_buttons
                        .spawn(style_prefab::settings_menu_button_row_node_bundle())
                        .with_children(|row| {
                            // Button for main volume
                            row.spawn((style_prefab::menu_button_bundle(), SettingsMenuButton))
                                .with_children(|button| {
                                    button.spawn(style_prefab::menu_button_text(
                                        "Game Scaling",
                                        fonts.interface_font.clone(),
                                    ));
                                });
                            // Text for main volume
                            row.spawn(style_prefab::settings_menu_info_text_bundle(
                                format!("{:.2}", video_settings.display_scale),
                                fonts.main_font.clone(),
                            ));
                        });
                    // Button and label for the video settings property hud_scale
                    menu_buttons
                        .spawn(style_prefab::settings_menu_button_row_node_bundle())
                        .with_children(|row| {
                            // Button for main volume
                            row.spawn((style_prefab::menu_button_bundle(), SettingsMenuButton))
                                .with_children(|button| {
                                    button.spawn(style_prefab::menu_button_text(
                                        "HUD Scaling",
                                        fonts.interface_font.clone(),
                                    ));
                                });
                            // Text for main volume
                            row.spawn(style_prefab::settings_menu_info_text_bundle(
                                format!("{:.2}", video_settings.hud_scale),
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
