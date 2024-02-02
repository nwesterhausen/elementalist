//! Has systems for the display settings menu.

use bevy::prelude::*;
use game_library::font_resource::FontResource;
use game_library::state::MenuState;

use crate::{despawn_with_tag, resources::style_prefab};

use super::{
    base::SettingsMenuEntity,
    button_actions::{ButtonAction, SettingsMenuButton},
};

/// Component for tagging entities that are part of the display settings menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
struct ControlsSettingsMenuEntity;

fn show_controls_settings(mut commands: Commands, fonts: Res<FontResource>) {
    commands
        .spawn((
            style_prefab::settings_menu_full_node_bundle(),
            ControlsSettingsMenuEntity,
            SettingsMenuEntity,
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

pub(super) struct ControlsSettingsMenuPlugin;

impl Plugin for ControlsSettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuState::Controls), show_controls_settings);
        // app.add_systems(
        //     Update,
        //     (handle_control_settings_changes,).run_if(in_state(MenuState::Controls)),
        // );
        app.add_systems(
            OnExit(MenuState::Controls),
            despawn_with_tag::<ControlsSettingsMenuEntity>,
        );
    }
}
