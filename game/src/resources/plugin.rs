use bevy::prelude::*;
use bevy_pkv::PkvStore;

use super::{update_cursor_position, CursorPosition};

pub struct ElementalistResourcesPlugin;

impl Plugin for ElementalistResourcesPlugin {
    fn build(&self, app: &mut App) {
        // ### ADD RESOURCES HERE ###
        app.insert_resource(CursorPosition::default());
        // Add a persistent key-value store for settings, etc.
        app.insert_resource(PkvStore::new("nwest.games", "elementalist"));

        // ### ADD SYSTEMS HERE ###
        app.add_systems(Update, update_cursor_position);
    }
}
