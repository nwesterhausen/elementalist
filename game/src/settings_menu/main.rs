//! Main menu systems.
use bevy::prelude::*;
use game_library::font_resource::FontResource;
use game_library::state::AppState;

use crate::style_prefab;

use super::{
    base::SettingsMenuEntity,
    button_actions::{SettingsButtons, SettingsMenuButton},
};

/// Tags for the (main) menu buttons/text/etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct BaseSettingsMenuEntity;

/// Show the main menu.
#[allow(clippy::too_many_lines)]
pub fn show_main_menu(
    mut commands: Commands,
    settings_buttons: Res<SettingsButtons>,
    fonts: Res<FontResource>,
    current_state: Res<State<AppState>>,
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
                            settings_buttons.show_audio_settings.clone(),
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
                            settings_buttons.show_video_settings.clone(),
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
                            settings_buttons.show_controls_settings.clone(),
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
                            settings_buttons.show_gameplay_settings.clone(),
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
                            settings_buttons.show_accessibility_settings.clone(),
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
                            settings_buttons.close.clone(),
                            SettingsMenuButton,
                        ))
                        .with_children(|button| {
                            button.spawn(style_prefab::menu_button_text(
                                "Back",
                                fonts.interface_font.clone(),
                            ));
                        });
                    // Back to MainMenu (show when we came from InGame)
                    if *current_state == AppState::InGame {
                        menu_buttons
                            .spawn((
                                style_prefab::menu_button_bundle(),
                                settings_buttons.quit_to_main_menu.clone(),
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
