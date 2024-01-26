//! Main menu systems.
use bevy::prelude::*;
use game_library::font_resource::FontResource;

use crate::common::colors;

use super::button_actions::ButtonAction;

/// Tags for the (main) menu buttons/text/etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct MainMenuEntity;

/// Show the main menu.
#[allow(clippy::too_many_lines)]
pub fn show_main_menu(mut commands: Commands, fonts: Res<FontResource>) {
    // Common style for all buttons on the screen
    let button_style = Style {
        margin: UiRect::px(10., 10., 0., 20.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Start,
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: colors::TEXT_COLOR,
        font: fonts.display_font.clone(),
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            },
            MainMenuEntity,
        ))
        .with_children(|parent| {
            // Game Title
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Settings",
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
                        align_items: AlignItems::Center,
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
                            ButtonAction::SettingsDisplay,
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
                            ButtonAction::CloseMenu,
                        ))
                        .with_children(|button| {
                            button
                                .spawn(TextBundle::from_section("Back", button_text_style.clone()));
                        });
                });
        });
}
