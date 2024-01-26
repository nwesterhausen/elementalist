//! Has systems for the display settings menu.

use bevy::prelude::*;
use game_library::{
    font_resource::FontResource, menu_helper::make_text_bundle, settings::VideoSettings,
};

use crate::common::colors;

use super::{
    base::{button_style, button_text, node_style, super_node_style},
    button_actions::ButtonAction,
};

/// Component for tagging entities that are part of the display settings menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(super) struct DisplaySettingsMenuEntity;

pub(super) fn show_display_settings(
    mut commands: Commands,
    fonts: Res<FontResource>,
    mut video_settings: ResMut<VideoSettings>,
) {
    commands
        .spawn((super_node_style(), DisplaySettingsMenuEntity))
        .with_children(|parent| {
            // Game Title
            parent.spawn(make_text_bundle(
                "Display Settings",
                fonts.display_font.clone(),
                72.0,
                colors::TEXT_COLOR,
                AlignSelf::Center,
            ));
            // #### MENU BUTTONS #####
            parent.spawn(node_style()).with_children(|menu_buttons| {
                // Change font settings button
                menu_buttons.spawn(button_style()).with_children(|button| {
                    button.spawn(button_text("Font", fonts.display_font.clone()));
                });
                // Back button (=> settings)
                menu_buttons
                    .spawn((button_style(), ButtonAction::BackToMenu))
                    .with_children(|button| {
                        button.spawn(button_text("Back", fonts.display_font.clone()));
                    });
            });
        });
}
