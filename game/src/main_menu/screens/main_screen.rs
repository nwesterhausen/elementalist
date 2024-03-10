use bevy::prelude::*;
use game_library::font_resource::FontResource;

use crate::{
    main_menu::{
        button_actions::{ButtonAction, MainMenuButton},
        plugin::OnMainMenuScreen,
    },
    style_prefab,
};

/// System to setup the main menu screen
///
/// When the main menu screen is entered, we spawn the main menu entities. This includes the
/// background, the title, and the buttons.
pub fn main_menu_setup(mut commands: Commands, fonts: Res<FontResource>) {
    // Common style for all buttons on the screen
    commands
        .spawn((style_prefab::main_menu_full_node_bundle(), OnMainMenuScreen))
        .with_children(|parent| {
            // Game Title
            parent.spawn(style_prefab::main_menu_title_bundle(
                "Elementalist",
                fonts.display_font.clone(),
            ));
            // #### MENU BUTTONS #####
            parent
                .spawn(style_prefab::main_menu_column_node_bundle())
                .with_children(|menu_buttons| {
                    // Start Game Button
                    menu_buttons
                        .spawn((
                            style_prefab::menu_button_bundle(),
                            ButtonAction::StartGame,
                            MainMenuButton,
                        ))
                        .with_children(|button| {
                            button.spawn(style_prefab::menu_button_text(
                                "Start",
                                fonts.interface_font.clone(),
                            ));
                        });

                    // Settings Button
                    menu_buttons
                        .spawn((
                            style_prefab::menu_button_bundle(),
                            ButtonAction::Settings,
                            MainMenuButton,
                        ))
                        .with_children(|button| {
                            button.spawn(style_prefab::menu_button_text(
                                "Settings",
                                fonts.interface_font.clone(),
                            ));
                        });

                    // Quit Button
                    menu_buttons
                        .spawn((
                            style_prefab::menu_button_bundle(),
                            ButtonAction::Quit,
                            MainMenuButton,
                        ))
                        .with_children(|button| {
                            button.spawn(style_prefab::menu_button_text(
                                "Quit",
                                fonts.interface_font.clone(),
                            ));
                        });
                });
        });
}
