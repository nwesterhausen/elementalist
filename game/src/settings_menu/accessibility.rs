//! Has systems for the display settings menu.

use bevy::prelude::*;
use game_library::{
    font_resource::{ChangeFont, FontChoice, FontResource},
    settings::{next_font_family, AccessibilitySettings, SettingCategory, SettingChanged},
};

use crate::common::buttons::style_prefab;

use super::{
    base::MenuEntity,
    button_actions::{ButtonAction, SettingsMenuButton},
    events::{ChangeSetting, IndividualSetting},
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
            style_prefab::settings_menu_full_node_bundle(),
            AccessibilitySettingsMenuEntity,
            MenuEntity,
        ))
        .with_children(|parent| {
            // Game Title
            parent.spawn(style_prefab::settings_menu_title_bundle(
                "Accessibility",
                fonts.display_font.clone(),
            ));
            // #### MENU BUTTONS #####
            parent
                .spawn(style_prefab::settings_menu_column_node_bundle())
                .with_children(|menu_buttons| {
                    // font choice (as a row with a label and a button)
                    menu_buttons
                        .spawn(style_prefab::settings_menu_button_row_node_bundle())
                        .with_children(|row| {
                            // Button for rotating font family
                            row.spawn((
                                style_prefab::menu_button_bundle(),
                                ButtonAction::RotateFontFamily,
                                SettingsMenuButton,
                            ))
                            .with_children(|button| {
                                button.spawn(style_prefab::menu_button_text(
                                    "Interface Font",
                                    fonts.interface_font.clone(),
                                ));
                            });
                            // Text for current font family
                            row.spawn(style_prefab::menu_button_text(
                                format!("{}", accessibility_settings.interface_font_family)
                                    .as_str(),
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

/// System to handle the accessibility menu button actions.
pub(super) fn handle_accessibility_setting_changes(
    mut er_change_setting: EventReader<ChangeSetting>,
    mut accessibility_settings: ResMut<AccessibilitySettings>,
    mut ew_setting_changed: EventWriter<SettingChanged>,
    mut ew_change_font: EventWriter<ChangeFont>,
    fonts: Res<FontResource>,
) {
    for change_setting in er_change_setting.read() {
        if matches!(change_setting.setting, IndividualSetting::FontFamily) {
            let new_font_family = next_font_family(accessibility_settings.interface_font_family);
            accessibility_settings.interface_font_family = new_font_family;
            ew_change_font.send(ChangeFont {
                font_choice: FontChoice::Interface,
                new_font: fonts.get_font_handle(new_font_family),
            });
            // Alert the system that the font has changed (to flush settings to disk)
            ew_setting_changed.send(SettingChanged(SettingCategory::Accessibility));
        }
    }
}
