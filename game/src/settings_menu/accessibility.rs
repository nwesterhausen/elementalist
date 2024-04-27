//! Has systems for the display settings menu.

use bevy::prelude::*;
use elementalist_game_library::{
    font_resource::{ChangeFont, FontChoice, FontResource},
    settings::{next_font_family, AccessibilitySettings, SettingCategory, SettingChanged},
    state::Settings,
};

use crate::{despawn_with_tag, style_prefab};

use super::{
    base::SettingsMenuEntity,
    button_actions::{SettingsButtons, SettingsMenuButton},
    events::{ChangeSetting, IndividualSetting},
};

/// Component for tagging entities that are part of the display settings menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
struct AccessibilitySettingsMenuEntity;

fn show_accessibility_settings(
    mut commands: Commands,
    fonts: Res<FontResource>,
    settings_buttons: Res<SettingsButtons>,
    accessibility_settings: Res<AccessibilitySettings>,
) {
    commands
        .spawn((
            style_prefab::settings_menu_full_node_bundle(),
            AccessibilitySettingsMenuEntity,
            SettingsMenuEntity,
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
                                settings_buttons.rotate_font_family.clone(),
                                SettingsMenuButton,
                            ))
                            .with_children(|button| {
                                button.spawn(style_prefab::menu_button_text(
                                    "Interface Font",
                                    fonts.interface_font.clone(),
                                ));
                            });
                            // Text for current font family
                            row.spawn((
                                style_prefab::settings_menu_info_text_bundle(
                                    accessibility_settings.interface_font_family,
                                    fonts.main_font.clone(),
                                ),
                                CurrentFontFamilyText,
                            ));
                        });
                    // Back button (=> settings)
                    menu_buttons
                        .spawn((
                            style_prefab::menu_button_bundle(),
                            settings_buttons.back_to_menu.clone(),
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

#[derive(Component)]
struct CurrentFontFamilyText;

pub(super) struct AccessibilitySettingsMenuPlugin;

impl Plugin for AccessibilitySettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(Settings::Accessibility),
            show_accessibility_settings,
        );
        app.add_systems(
            Update,
            (
                handle_accessibility_setting_changes,
                (update_current_font_family_text),
            )
                .run_if(in_state(Settings::Accessibility)),
        );
        app.add_systems(
            OnExit(Settings::Accessibility),
            despawn_with_tag::<AccessibilitySettingsMenuEntity>,
        );
    }
}

/// System to handle the accessibility menu button actions.
fn handle_accessibility_setting_changes(
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

/// System to update the text of the current font family.
fn update_current_font_family_text(
    accessibility_settings: Res<AccessibilitySettings>,
    fonts: Res<FontResource>,
    mut text_query: Query<&mut Text, With<CurrentFontFamilyText>>,
) {
    for mut text in &mut text_query {
        text.sections[0].value = accessibility_settings.interface_font_family.into();
        // Set the font to reflect the new font family
        text.sections[0].style.font =
            fonts.get_font_handle(accessibility_settings.interface_font_family);
    }
}
