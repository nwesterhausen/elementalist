use bevy::prelude::*;
use bevy_health_bar3d::prelude::{ColorScheme, ForegroundColor, HealthBarPlugin};
use bevy_pkv::PkvStore;

use game_library::{
    font_resource::{change_font, ChangeFont, FontResource},
    CursorPosition, Health, Mana, SkillTrack, SpellChoices,
};

use crate::{
    app_systems,
    common::{buttons, colors},
};

use super::{
    cleanup::cleanup_then_exit, cursor_position::update_cursor_position, fonts::set_initial_fonts,
    spritesheet::load_spell_atlas, AppState, ReturnToState,
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

        // health_bar3d plugin and resources (like colors for the bars)
        app.add_plugins((
            HealthBarPlugin::<Health>::default(),
            HealthBarPlugin::<Mana>::default(),
            HealthBarPlugin::<SkillTrack>::default(),
        ))
        .insert_resource(
            ColorScheme::<Health>::new()
                .foreground_color(ForegroundColor::Static(colors::HEALTH_BAR_COLOR)),
        )
        .insert_resource(
            ColorScheme::<Mana>::new()
                .foreground_color(ForegroundColor::Static(colors::MANA_BAR_COLOR)),
        )
        .insert_resource(
            ColorScheme::<SkillTrack>::new()
                .foreground_color(ForegroundColor::Static(colors::SKILL_TRACK_BAR_COLOR)),
        );

        // ### ADD EVENTS HERE ###
        app.add_event::<ChangeFont>();

        // ### ADD RESOURCES HERE ###
        app.init_resource::<ReturnToState>()
            .insert_resource(CursorPosition::default())
            .insert_resource(SpellChoices::default())
            .insert_resource(FontResource::default())
            .insert_resource(PkvStore::new("games.nwest.one", "Elementalist"));

        // ### GRAPHICS RESOURCES ###
        app.add_systems(Startup, load_spell_atlas);

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
