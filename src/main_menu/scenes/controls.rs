use bevy::prelude::*;

use super::super::{buttons::ButtonAction, colors, tags};

/// System to setup the controls settings screen
///
/// This screen allows the player to change the controls for the game.
///
/// Todo: Need to grab current controls from world and display them on the screen
/// Todo: Need to save the controls to the world when the player changes them
/// Todo: Need to reset the controls to default when the player clicks the reset button
pub fn controls_settings_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Common style for all buttons on the screen
    let button_style = Style {
        height: Val::Px(65.0),
        padding: UiRect::all(Val::Px(5.0)),
        align_items: AlignItems::Start,
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: colors::TEXT,
        font: asset_server.load("ui/fonts/AlmendraDisplay-Regular.ttf"),
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    flex_wrap: FlexWrap::Wrap,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            },
            tags::OnControlsSettingsMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::ControlsUp,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Up", button_text_style.clone()));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::ControlsDown,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Down", button_text_style.clone()));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::ControlsLeft,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Left", button_text_style.clone()));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::ControlsRight,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Right", button_text_style.clone()));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::ControlsPrimaryCast,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Primary Cast",
                        button_text_style.clone(),
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::ControlsSecondaryCast,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Secondary Cast",
                        button_text_style.clone(),
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::ControlsDefensiveCast,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Defensive Cast",
                        button_text_style.clone(),
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::ControlsUltimateCast,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Ultimate Cast",
                        button_text_style.clone(),
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::ControlsPause,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Pause", button_text_style.clone()));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::ControlsInteract,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Interact",
                        button_text_style.clone(),
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::ControlsBack,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Cancel",
                        button_text_style.clone(),
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::ControlsAutoAttack,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Auto Attack",
                        button_text_style.clone(),
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::ControlsAutoAim,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Auto Aim",
                        button_text_style.clone(),
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::ControlsReset,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Reset", button_text_style.clone()));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::BackToSettings,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Back", button_text_style.clone()));
                });
        });
}
