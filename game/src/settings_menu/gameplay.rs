//! Has systems for the display settings menu.

use bevy::prelude::*;
use game_library::{
    font_resource::FontResource,
    settings::{GameplaySettings, SettingCategory, SettingChanged},
};

use crate::common::colors;

use super::{
    base::{button_style, button_text},
    button_actions::ButtonAction,
    events::{ChangeSetting, IndividualSetting},
};

/// Component for tagging entities that are part of the display settings menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(super) struct GameplaySettingsMenuEntity;

pub(super) fn show_gameplay_settings(
    mut commands: Commands,
    fonts: Res<FontResource>,
    gameplay_settings: Res<GameplaySettings>,
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
                    // Auto-aim button (as a row with a label and a button)
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
                            // Button for auto-aim
                            row.spawn((button_style(), ButtonAction::ToggleAutoAim))
                                .with_children(|button| {
                                    button.spawn(button_text(
                                        "Auto-Aim",
                                        fonts.interface_font.clone(),
                                    ));
                                });
                            // Text for auto-aim
                            row.spawn(button_text(
                                format!("{}", gameplay_settings.auto_aim).as_str(),
                                fonts.main_font.clone(),
                            ));
                        });
                    // Auto-cast button (as a row with a label and a button)
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
                            // Button for auto-cast
                            row.spawn((button_style(), ButtonAction::ToggleAutoCast))
                                .with_children(|button| {
                                    button.spawn(button_text(
                                        "Auto-Cast",
                                        fonts.interface_font.clone(),
                                    ));
                                });
                            // Text for auto-cast
                            row.spawn(button_text(
                                format!("{}", gameplay_settings.auto_cast).as_str(),
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

/// System to handle the gameplay menu button actions.
pub(super) fn handle_gameplay_setting_changes(
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
