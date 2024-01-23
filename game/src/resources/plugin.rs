use bevy::prelude::*;

use game_library::{
    font_resource::{change_font, ChangeFont, FontResource},
    CursorPosition, SpellChoices,
};

use super::{
    cursor_position::update_cursor_position, fonts::set_initial_fonts,
    spritesheet::load_spell_atlas,
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

        // ### GRAPHICS RESOURCES ###
        app.add_systems(Startup, load_spell_atlas);

        // ### ADD SYSTEMS HERE ###
        app.add_systems(Startup, set_initial_fonts);
        app.add_systems(Update, (update_cursor_position, change_font));
    }
}
