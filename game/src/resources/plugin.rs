use bevy::prelude::*;
use bevy_pkv::PkvStore;

use game_library::{
    font_resource::{change_font, ChangeFont, FontResource},
    CursorPosition, SpellChoices,
};

use super::{
    cursor_position::update_cursor_position, fonts::set_initial_fonts,
    spritesheet::load_spell_atlas, ReturnToState,
};

pub struct ElementalistResourcesPlugin;

impl Plugin for ElementalistResourcesPlugin {
    fn build(&self, app: &mut App) {
        // ### ADD EVENTS HERE ###
        app.add_event::<ChangeFont>();

        // ### ADD RESOURCES HERE ###
        app.insert_resource(CursorPosition::default());
        app.insert_resource(SpellChoices::default());
        app.insert_resource(FontResource::default());
        app.init_resource::<ReturnToState>();
        app.insert_resource(PkvStore::new("games.nwest.one", "Elementalist"));

        // ### GRAPHICS RESOURCES ###
        app.add_systems(Startup, load_spell_atlas);

        // ### ADD SYSTEMS HERE ###
        app.add_systems(PostStartup, set_initial_fonts);
        app.add_systems(Update, (update_cursor_position, change_font));
    }
}
