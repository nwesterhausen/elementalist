//! Has systems for the display settings menu.

use bevy::prelude::*;
use game_library::font_resource::FontResource;

use crate::common::buttons::style_prefab;

use super::{
    base::MenuEntity,
    button_actions::{ButtonAction, SettingsMenuButton},
};

/// Component for tagging entities that are part of the display settings menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(super) struct ControlsSettingsMenuEntity;

pub(super) fn show_controls_settings(mut commands: Commands, fonts: Res<FontResource>) {
    commands
        .spawn((
            style_prefab::settings_menu_full_node_bundle(),
            ControlsSettingsMenuEntity,
            MenuEntity,
        ))
        .with_children(|parent| {
            // Menu Title
            parent.spawn(style_prefab::settings_menu_title_bundle(
                "Controls",
                fonts.display_font.clone(),
            ));
            // #### MENU BUTTONS #####
            parent
                .spawn(style_prefab::settings_menu_column_node_bundle())
                .with_children(|menu_buttons| {
                    // Back button (=> settings)
                    menu_buttons
                        .spawn((
                            style_prefab::menu_button_bundle(),
                            ButtonAction::BackToMenu,
                            SettingsMenuButton,
                        ))
                        .with_children(|button| {
                            button.spawn(style_prefab::menu_button_text(
                                "Back",
                                fonts.interface_font.clone(),
                            ));
                        });
                });
        });
}
