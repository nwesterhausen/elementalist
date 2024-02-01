//! Main menu systems.
use bevy::prelude::*;
use game_library::font_resource::FontResource;

use crate::{
    resources::style_prefab,
    resources::{AppState, ReturnToState},
};

use super::{
    base::SettingsMenuEntity,
    button_actions::{ButtonAction, SettingsMenuButton},
};

/// Tags for the (main) menu buttons/text/etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct BaseSettingsMenuEntity;

/// Show the main menu.
#[allow(clippy::too_many_lines)]
pub fn show_main_menu(
    mut commands: Commands,
    fonts: Res<FontResource>,
    return_to_state: Res<ReturnToState>,
) {
    commands
        .spawn((
            style_prefab::settings_menu_full_node_bundle(),
            BaseSettingsMenuEntity,
            SettingsMenuEntity,
        ))
        .with_children(|parent| {
            // Game Title
            parent.spawn(style_prefab::settings_menu_title_bundle(
                "Settings",
                fonts.display_font.clone(),
            ));
            // #### MENU BUTTONS #####
            parent
                .spawn(style_prefab::settings_menu_column_node_bundle())
                .with_children(|menu_buttons| {
                    // Audio button
                    menu_buttons
                        .spawn((
                            style_prefab::menu_button_bundle(),
                            ButtonAction::SettingsAudio,
                            SettingsMenuButton,
                        ))
                        .with_children(|button| {
                            button.spawn(style_prefab::menu_button_text(
                                "Audio",
                                fonts.interface_font.clone(),
                            ));
                        });
                    // Video button
                    menu_buttons
                        .spawn((
                            style_prefab::menu_button_bundle(),
                            ButtonAction::SettingsDisplay,
                            SettingsMenuButton,
                        ))
                        .with_children(|button| {
                            button.spawn(style_prefab::menu_button_text(
                                "Video",
                                fonts.interface_font.clone(),
                            ));
                        });
                    // Controls button
                    menu_buttons
                        .spawn((
                            style_prefab::menu_button_bundle(),
                            ButtonAction::SettingsControls,
                            SettingsMenuButton,
                        ))
                        .with_children(|button| {
                            button.spawn(style_prefab::menu_button_text(
                                "Controls",
                                fonts.interface_font.clone(),
                            ));
                        });
                    // Gameplay button
                    menu_buttons
                        .spawn((
                            style_prefab::menu_button_bundle(),
                            ButtonAction::SettingsGameplay,
                            SettingsMenuButton,
                        ))
                        .with_children(|button| {
                            button.spawn(style_prefab::menu_button_text(
                                "Gameplay",
                                fonts.interface_font.clone(),
                            ));
                        });
                    // Accessibility button
                    menu_buttons
                        .spawn((
                            style_prefab::menu_button_bundle(),
                            ButtonAction::SettingsAccessibility,
                            SettingsMenuButton,
                        ))
                        .with_children(|button| {
                            button.spawn(style_prefab::menu_button_text(
                                "Accessibility",
                                fonts.interface_font.clone(),
                            ));
                        });
                    // Back button (=> return to where we came from)
                    menu_buttons
                        .spawn((
                            style_prefab::menu_button_bundle(),
                            ButtonAction::CloseMenu,
                            SettingsMenuButton,
                        ))
                        .with_children(|button| {
                            button.spawn(style_prefab::menu_button_text(
                                "Back",
                                fonts.interface_font.clone(),
                            ));
                        });
                    // Back to MainMenu (show when we came from InGame)
                    if return_to_state.0 == AppState::InGame {
                        menu_buttons
                            .spawn((
                                style_prefab::menu_button_bundle(),
                                ButtonAction::GoToMainGameMenu,
                                SettingsMenuButton,
                            ))
                            .with_children(|button| {
                                button.spawn(style_prefab::menu_button_text(
                                    "Quit to Main Menu",
                                    fonts.interface_font.clone(),
                                ));
                            });
                    }
                });
        });
}
