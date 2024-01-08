use bevy::prelude::*;

use super::{update_cursor_position_resource, OffsetCursorPosition};

pub struct ElementalistResourcesPlugin;

impl Plugin for ElementalistResourcesPlugin {
    fn build(&self, app: &mut App) {
        // ### ADD RESOURCES HERE ###
        app.insert_resource(OffsetCursorPosition::default());

        // ### ADD SYSTEMS HERE ###
        app.add_systems(Update, update_cursor_position_resource);
    }
}
