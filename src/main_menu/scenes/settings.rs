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
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_style = Style {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
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
                    ButtonAction::SoundSettings,
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
                    ButtonAction::DisplaySettings,
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
                    ButtonAction::ControlsSettings,
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
                    ButtonAction::GameplaySettings,
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
                    ButtonAction::BackToMain,
                ))
                .with_children(|parent| {
                    let icon = asset_server.load("ui/icons/arrow-badge-left.png");
                    parent.spawn(ImageBundle {
                        style: button_icon_style,
                        image: UiImage::new(icon),
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section("Back", button_text_style.clone()));
                });
        });
}
