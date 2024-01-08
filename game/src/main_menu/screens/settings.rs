use bevy::prelude::*;

use crate::main_menu::{button_actions::ButtonAction, components::OnSettingsMenuScreen};

/// System to setup the main menu screen
///
/// When the main menu screen is entered, we spawn the main menu entities. This includes the
/// background, the title, and the buttons.
pub fn settings_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Common style for all buttons on the screen
    let button_style = Style {
        margin: UiRect::px(10., 10., 0., 20.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Start,
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: Color::WHITE,
        font: asset_server.load("ui/fonts/AlmendraDisplay-Regular.ttf"),
        ..default()
    };

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
            OnSettingsMenuScreen,
        ))
        .with_children(|parent| {
            // Game Title
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Settings",
                    TextStyle {
                        font: asset_server.load("ui/fonts/AlmendraDisplay-Regular.ttf"),
                        font_size: 72.0,
                        color: Color::WHITE,
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
                    // Audio button
                    menu_buttons
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: Color::NONE.into(),
                                ..default()
                            },
                            ButtonAction::SettingsAudio,
                        ))
                        .with_children(|button| {
                            button.spawn(TextBundle::from_section(
                                "Audio",
                                button_text_style.clone(),
                            ));
                        });
                    // Video button
                    menu_buttons
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: Color::NONE.into(),
                                ..default()
                            },
                            ButtonAction::SettingsVideo,
                        ))
                        .with_children(|button| {
                            button.spawn(TextBundle::from_section(
                                "Video",
                                button_text_style.clone(),
                            ));
                        });
                    // Controls button
                    menu_buttons
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: Color::NONE.into(),
                                ..default()
                            },
                            ButtonAction::SettingsControls,
                        ))
                        .with_children(|button| {
                            button.spawn(TextBundle::from_section(
                                "Controls",
                                button_text_style.clone(),
                            ));
                        });
                    // Gameplay button
                    menu_buttons
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: Color::NONE.into(),
                                ..default()
                            },
                            ButtonAction::SettingsGameplay,
                        ))
                        .with_children(|button| {
                            button.spawn(TextBundle::from_section(
                                "Gameplay",
                                button_text_style.clone(),
                            ));
                        });
                    // Back button (=> main menu)
                    menu_buttons
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: Color::NONE.into(),
                                ..default()
                            },
                            ButtonAction::BackToMenu,
                        ))
                        .with_children(|button| {
                            button
                                .spawn(TextBundle::from_section("Back", button_text_style.clone()));
                        });
                });
        });
}
