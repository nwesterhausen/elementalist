//! Has systems for the display settings menu.

use bevy::prelude::*;
use game_library::{
    font_resource::FontResource,
    settings::{GameplaySettings, SettingCategory, SettingChanged},
};

use crate::{common::buttons::style_prefab, despawn_with_tag};

use super::{
    base::SettingsMenuEntity,
    button_actions::{ButtonAction, SettingsMenuButton},
    events::{ChangeSetting, IndividualSetting},
    MenuState,
};

/// Component for tagging entities that are part of the display settings menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
struct GameplaySettingsMenuEntity;

fn show_gameplay_settings(
    mut commands: Commands,
    fonts: Res<FontResource>,
    gameplay_settings: Res<GameplaySettings>,
) {
    commands
        .spawn((
            style_prefab::settings_menu_full_node_bundle(),
            GameplaySettingsMenuEntity,
            SettingsMenuEntity,
        ))
        .with_children(|parent| {
            // Game Title
            parent.spawn(style_prefab::settings_menu_title_bundle(
                "Gameplay Settings",
                fonts.display_font.clone(),
            ));
            // #### MENU BUTTONS #####
            parent
                .spawn(style_prefab::settings_menu_column_node_bundle())
                .with_children(|menu_buttons| {
                    // Auto-aim button (as a row with a label and a button)
                    menu_buttons
                        .spawn(style_prefab::settings_menu_button_row_node_bundle())
                        .with_children(|row| {
                            // Button for auto-aim
                            row.spawn((
                                style_prefab::menu_button_bundle(),
                                ButtonAction::ToggleAutoAim,
                                SettingsMenuButton,
                            ))
                            .with_children(|button| {
                                button.spawn(style_prefab::menu_button_text(
                                    "Auto-Aim",
                                    fonts.interface_font.clone(),
                                ));
                            });
                            // Text for auto-aim
                            row.spawn((
                                style_prefab::settings_menu_info_text_bundle(
                                    format!("{}", gameplay_settings.auto_aim).as_str(),
                                    fonts.main_font.clone(),
                                ),
                                CurrentAutoAimStateText,
                            ));
                        });
                    // Auto-cast button (as a row with a label and a button)
                    menu_buttons
                        .spawn(style_prefab::settings_menu_button_row_node_bundle())
                        .with_children(|row| {
                            // Button for auto-cast
                            row.spawn((
                                style_prefab::menu_button_bundle(),
                                ButtonAction::ToggleAutoCast,
                                SettingsMenuButton,
                            ))
                            .with_children(|button| {
                                button.spawn(style_prefab::menu_button_text(
                                    "Auto-Cast",
                                    fonts.interface_font.clone(),
                                ));
                            });
                            // Text for auto-cast
                            row.spawn((
                                style_prefab::settings_menu_info_text_bundle(
                                    format!("{}", gameplay_settings.auto_cast).as_str(),
                                    fonts.main_font.clone(),
                                ),
                                CurrentAutoCastStateText,
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

pub(super) struct GameplaySettingsMenuPlugin;

impl Plugin for GameplaySettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuState::Gameplay), show_gameplay_settings);
        app.add_systems(
            Update,
            (
                handle_gameplay_setting_changes,
                (
                    update_current_auto_aim_state_text,
                    update_current_auto_cast_state_text,
                ),
            )
                .run_if(in_state(MenuState::Gameplay)),
        );
        app.add_systems(
            OnExit(MenuState::Gameplay),
            despawn_with_tag::<GameplaySettingsMenuEntity>,
        );
    }
}

/// System to handle the gameplay menu button actions.
fn handle_gameplay_setting_changes(
    mut er_change_setting: EventReader<ChangeSetting>,
    mut gameplay_settings: ResMut<GameplaySettings>,
    mut ew_setting_changed: EventWriter<SettingChanged>,
) {
    for change_setting in er_change_setting.read() {
        match change_setting.setting {
            IndividualSetting::AutoCast => {
                gameplay_settings.auto_cast = !gameplay_settings.auto_cast;
                // Alert the system that the font has changed (to flush settings to disk)
                ew_setting_changed.send(SettingChanged(SettingCategory::Gameplay));
            }
            IndividualSetting::AutoAim => {
                gameplay_settings.auto_aim = !gameplay_settings.auto_aim;
                // Alert the system that the font has changed (to flush settings to disk)
                ew_setting_changed.send(SettingChanged(SettingCategory::Gameplay));
            }
            _ => {}
        }
    }
}

#[derive(Component)]
struct CurrentAutoAimStateText;

#[derive(Component)]
struct CurrentAutoCastStateText;

/// System to update the text for the current auto-aim state.
fn update_current_auto_aim_state_text(
    mut text_query: Query<(&mut Text, &CurrentAutoAimStateText)>,
    gameplay_settings: Res<GameplaySettings>,
) {
    for (mut text, _tag) in &mut text_query {
        text.sections[0].value = format!("{}", gameplay_settings.auto_aim);
    }
}

/// System to update the text for the current auto-cast state.
fn update_current_auto_cast_state_text(
    mut text_query: Query<(&mut Text, &CurrentAutoCastStateText)>,
    gameplay_settings: Res<GameplaySettings>,
) {
    for (mut text, _tag) in &mut text_query {
        text.sections[0].value = format!("{}", gameplay_settings.auto_cast);
    }
}
