use bevy::prelude::*;
use bevy_pkv::PkvStore;

use game_library::{
    font_resource::{change_font, FontResource},
    CursorPosition, SpellChoices,
};

use super::{cursor_position::update_cursor_position, spritesheet::load_spell_atlas};

pub struct ElementalistResourcesPlugin;

impl Plugin for ElementalistResourcesPlugin {
    fn build(&self, app: &mut App) {
        // ### ADD RESOURCES HERE ###
        app.insert_resource(CursorPosition::default());
        app.insert_resource(SpellChoices::default());
        app.insert_resource(FontResource::default());
        // Add a persistent key-value store for settings, etc.
        app.insert_resource(PkvStore::new("nwest.games", "elementalist"));

        // ### GRAPHICS RESOURCES ###
        app.add_systems(Startup, load_spell_atlas);

        // ### ADD SYSTEMS HERE ###
        app.add_systems(Update, (update_cursor_position, change_font));
    }
}
