use bevy::prelude::*;

use bevy_hanabi::HanabiPlugin;
use game_library::{
    buttons,
    data_loader::DataLoaderPlugin,
    font_resource::{change_font, ChangeFont, FontResource},
    progress_bar::ProgressBarPlugin,
    settings::SettingsPlugin,
    spells::SpellSelection,
    state::{AppState, Game, Save, Settings},
    CursorPosition, Health, Mana, Xp,
};

use crate::app_systems;

use super::{
    cleanup::cleanup_then_exit, cursor_position::update_cursor_position, fonts::set_initial_fonts,
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
        // The app states
        app.init_state::<AppState>()
            .init_state::<Game>()
            .init_state::<Settings>()
            .init_state::<Save>();

        // Data loading plugin
        app.add_plugins(DataLoaderPlugin);

        // ProgressBar plugins are used to display health or experience bars. This might be out of
        // "resource" scope.
        app.add_plugins((
            ProgressBarPlugin::<Health>::default(),
            ProgressBarPlugin::<Xp>::default(),
            ProgressBarPlugin::<Mana>::default(),
        ));

        // Add the Hanabi plugin (note that the particle effects are loaded from disk in the `DataLoaderPlugin`)
        app.add_plugins(HanabiPlugin);

        app
            // Game settings and resources
            .add_plugins(
                SettingsPlugin::default().with_structure(&"games.nwest.one", &"Elementalist"),
            );

        app
            // The cursor position resource, used to aim spells or know cursor coordinates easily
            .insert_resource(CursorPosition::default())
            // The player's spell choices
            .insert_resource(SpellSelection::default())
            // The font resource has handles to the fonts used in the game to save loading assets constantly
            // and to easily allow the user to change the font (e.g. for accessibility)
            .add_event::<ChangeFont>()
            .insert_resource(FontResource::default());

        app.add_systems(
            Startup,
            (
                // Draws a mostly transparent version and build git hash in the top right corner
                app_systems::add_game_descriptor,
            ),
        )
        .add_systems(
            Update,
            (
                // Updates the cursor position (should we have a cursor position plugin?)
                update_cursor_position,
                // Changes the font when the user changes the font (responds to [`ChangeFont`])
                change_font,
                // Handles the button VISUAL interactions (e.g. hover, click, pressed, etc.)
                // Also handles the logic
                buttons::handle_button_events,
            ),
        )
        .add_systems(
            PostStartup,
            // This system is [`PostStartup`] because we need to load any font settings from disk before assigning the
            // initial fonts handles.
            set_initial_fonts,
        )
        .add_systems(
            OnEnter(AppState::CleanUp),
            // Cleanup the game when we exit (e.g. save settings, etc.)
            cleanup_then_exit,
        );
    }
}
