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
        justify_content: JustifyContent::SpaceBetween,
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: colors::TEXT,
        font: asset_server.load("ui/fonts/AlmendraDisplay-Regular.ttf"),
        ..default()
    };
    let current_control_button_text_style = TextStyle {
        font_size: 32.0,
        color: Color::CRIMSON,
        font: asset_server.load("ui/fonts/Almendra-Bold.ttf"),
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
                    ButtonAction::SetControlsUp,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Up", button_text_style.clone()));
                    parent.spawn(TextBundle::from_section(
                        "W",
                        current_control_button_text_style.clone(),
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::SetControlsDown,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Down", button_text_style.clone()));
                    parent.spawn(TextBundle::from_section(
                        "S",
                        current_control_button_text_style.clone(),
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::SetControlsLeft,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Left", button_text_style.clone()));
                    parent.spawn(TextBundle::from_section(
                        "A",
                        current_control_button_text_style.clone(),
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::SetControlsRight,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Right", button_text_style.clone()));
                    parent.spawn(TextBundle::from_section(
                        "D",
                        current_control_button_text_style.clone(),
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::SetControlsPrimaryCast,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Primary Cast",
                        button_text_style.clone(),
                    ));
                    parent.spawn(TextBundle::from_section(
                        "Left Mouse Button",
                        current_control_button_text_style.clone(),
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::SetControlsSecondaryCast,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Secondary Cast",
                        button_text_style.clone(),
                    ));
                    parent.spawn(TextBundle::from_section(
                        "Right Mouse Button",
                        current_control_button_text_style.clone(),
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::SetControlsDefensiveCast,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Defensive Cast",
                        button_text_style.clone(),
                    ));
                    parent.spawn(TextBundle::from_section(
                        "Space",
                        current_control_button_text_style.clone(),
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::SetControlsUltimateCast,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Ultimate Cast",
                        button_text_style.clone(),
                    ));
                    parent.spawn(TextBundle::from_section(
                        "E",
                        current_control_button_text_style.clone(),
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::SetControlsPause,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Pause", button_text_style.clone()));
                    parent.spawn(TextBundle::from_section(
                        "Tab",
                        current_control_button_text_style.clone(),
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::SetControlsAcceptInteract,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Interact/Accept",
                        button_text_style.clone(),
                    ));
                    parent.spawn(TextBundle::from_section(
                        "F",
                        current_control_button_text_style.clone(),
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::SetControlsBackCancel,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Cancel/Back",
                        button_text_style.clone(),
                    ));
                    parent.spawn(TextBundle::from_section(
                        "X",
                        current_control_button_text_style.clone(),
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::SetControlsAutoAttack,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Auto Attack",
                        button_text_style.clone(),
                    ));
                    parent.spawn(TextBundle::from_section(
                        "Q",
                        current_control_button_text_style.clone(),
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::SetControlsAutoAim,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Auto Aim",
                        button_text_style.clone(),
                    ));
                    parent.spawn(TextBundle::from_section(
                        "T",
                        current_control_button_text_style.clone(),
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::ResetControlsToDefault,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Reset to Defaults",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::GOLD,
                            font: asset_server.load("ui/fonts/Almendra-Bold.ttf"),
                            ..default()
                        },
                    ));
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
                    parent.spawn(TextBundle::from_section(
                        "Back",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::ANTIQUE_WHITE,
                            font: asset_server.load("ui/fonts/Almendra-Bold.ttf"),
                            ..default()
                        },
                    ));
                });
        });
}
