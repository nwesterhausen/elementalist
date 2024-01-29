use bevy::prelude::*;
use bevy_pkv::PkvStore;

use game_library::{
    font_resource::{change_font, ChangeFont, FontResource},
    progress_bar::ProgressBarPlugin,
    CursorPosition, Health, SpellChoices,
};

use crate::{app_systems, resources::buttons};

use super::{
    cleanup::cleanup_then_exit, cursor_position::update_cursor_position, fonts::set_initial_fonts,
    spritesheet::LoadSpritesheetPlugin, AppState, ReturnToState,
};

/// Elementalist resources plugin. This loads the resources needed which may not be
/// specific to a single module. This includes:
///
/// - App state
/// - System to display version and build information (top right)
/// - System to load data from `game_data` directory
/// - Fonts
///    - System to set the initial fonts
///    - System to change the fonts
/// - Cursor position
///    - System to update the cursor position
/// - The settings database on disk
/// - The spell atlas (the spritesheet for the spells)
/// - The spell choices resource (this probably should be moved to the Player plugin)
/// - A state to return to when the settings menu is closed
pub struct ElementalistResourcesPlugin;

impl Plugin for ElementalistResourcesPlugin {
    fn build(&self, app: &mut App) {
        // The app state, to track if in menu, in game, etc.
        app.add_state::<AppState>();

        app.add_plugins(ProgressBarPlugin::<Health>::default());

        // ### ADD EVENTS HERE ###
        app.add_event::<ChangeFont>();

        // ### ADD RESOURCES HERE ###
        app.init_resource::<ReturnToState>()
            .insert_resource(CursorPosition::default())
            .insert_resource(SpellChoices::default())
            .insert_resource(FontResource::default())
            .insert_resource(PkvStore::new("games.nwest.one", "Elementalist"));

        // ### GRAPHICS RESOURCES ###
        app.add_plugins(LoadSpritesheetPlugin);

        // ### ADD SYSTEMS HERE ###
        app.add_systems(
            Startup,
            (
                app_systems::add_game_descriptor,
                app_systems::load_data_file_dir,
            ),
        )
        .add_systems(
            Update,
            (
                update_cursor_position,
                change_font,
                buttons::interaction_system,
            ),
        )
        // This system is [`PostStartup`] because we need to load any font settings from
        // disk before assigning the initial fonts handles.
        .add_systems(PostStartup, set_initial_fonts)
        // handle the cleanup state
        .add_systems(OnEnter(AppState::CleanUp), cleanup_then_exit);
    }
}
