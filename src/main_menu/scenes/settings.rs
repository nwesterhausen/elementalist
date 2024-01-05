use bevy::prelude::*;

use super::super::{buttons::ButtonAction, colors, tags};

/// System to setup the settings screen
pub fn settings_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
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
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            tags::OnSettingsMenuScreen,
        ))
        .with_children(|parent| {
            // A sub-menu for settings with an entry for each setting category
            // (audio, video, controls, gameplay)
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::SettingsSound,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Sound", button_text_style.clone()));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::SettingsDisplay,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Display",
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
                    ButtonAction::SettingsControls,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Controls",
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
                    ButtonAction::SettingsGameplay,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Gameplay",
                        button_text_style.clone(),
                    ));
                });
            // A back button to go back to the main menu
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: colors::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonAction::BackToMainMenu,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Back", button_text_style.clone()));
                });
        });
}
