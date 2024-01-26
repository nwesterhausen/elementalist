//! Has systems for the display settings menu.

use bevy::prelude::*;
use game_library::{font_resource::FontResource, menu_helper::make_text_bundle};

use crate::common::colors;

use super::{
    base::{button_style, button_text, node_style, super_node_style},
    button_actions::ButtonAction,
};

/// Component for tagging entities that are part of the display settings menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(super) struct ControlsSettingsMenuEntity;

pub(super) fn show_controls_settings(mut commands: Commands, fonts: Res<FontResource>) {
    commands
        .spawn((super_node_style(), ControlsSettingsMenuEntity))
        .with_children(|parent| {
            // Menu Title
            parent.spawn(make_text_bundle(
                "Controls",
                fonts.display_font.clone(),
                72.0,
                colors::TEXT_COLOR,
                AlignSelf::Center,
            ));
            // #### MENU BUTTONS #####
            parent.spawn(node_style()).with_children(|menu_buttons| {
                // Back button (=> settings)
                menu_buttons
                    .spawn((button_style(), ButtonAction::BackToMenu))
                    .with_children(|button| {
                        button.spawn(button_text("Back", fonts.interface_font.clone()));
                    });
            });
        });
}
