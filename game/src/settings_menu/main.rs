//! Main menu systems.
use bevy::prelude::*;
use game_library::{font_resource::FontResource, menu_helper::make_text_bundle};

use crate::common::colors;

use super::{
    base::{button_style, button_text, node_style, super_node_style},
    button_actions::ButtonAction,
};

/// Tags for the (main) menu buttons/text/etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct MainMenuEntity;

/// Show the main menu.
pub fn show_main_menu(mut commands: Commands, fonts: Res<FontResource>) {
    commands
        .spawn((super_node_style(), MainMenuEntity))
        .with_children(|parent| {
            // Game Title
            parent.spawn(make_text_bundle(
                "Settings",
                fonts.display_font.clone(),
                72.0,
                colors::TEXT_COLOR,
                AlignSelf::Center,
            ));
            // #### MENU BUTTONS #####
            parent.spawn(node_style()).with_children(|menu_buttons| {
                // Audio button
                menu_buttons
                    .spawn((button_style(), ButtonAction::SettingsAudio))
                    .with_children(|button| {
                        button.spawn(button_text("Audio", fonts.interface_font.clone()));
                    });
                // Video button
                menu_buttons
                    .spawn((button_style(), ButtonAction::SettingsDisplay))
                    .with_children(|button| {
                        button.spawn(button_text("Video", fonts.interface_font.clone()));
                    });
                // Controls button
                menu_buttons
                    .spawn((button_style(), ButtonAction::SettingsControls))
                    .with_children(|button| {
                        button.spawn(button_text("Controls", fonts.interface_font.clone()));
                    });
                // Gameplay button
                menu_buttons
                    .spawn((button_style(), ButtonAction::SettingsGameplay))
                    .with_children(|button| {
                        button.spawn(button_text("Gameplay", fonts.interface_font.clone()));
                    });
                // Accessibility button
                menu_buttons
                    .spawn((button_style(), ButtonAction::SettingsAccessibility))
                    .with_children(|button| {
                        button.spawn(button_text("Accessibility", fonts.interface_font.clone()));
                    });
                // Back button (=> main menu)
                menu_buttons
                    .spawn((button_style(), ButtonAction::CloseMenu))
                    .with_children(|button| {
                        button.spawn(button_text("Back", fonts.interface_font.clone()));
                    });
            });
        });
}
