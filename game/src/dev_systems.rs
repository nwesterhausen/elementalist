use bevy::prelude::*;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use game_library::{Attribute, Skill, Stat, StatBonus, Volume};

use crate::{
    common::{
        movement::{Acceleration, Velocity},
        stats::{Health, Mana},
    },
    resources::{CursorPosition, SpellChoices},
};

pub struct DevSystemsPlugin;

impl Plugin for DevSystemsPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app.add_plugins(WorldInspectorPlugin::new())
            .register_type::<Attribute>()
            .register_type::<Skill>()
            .register_type::<Stat>()
            .register_type::<StatBonus>()
            .register_type::<Volume>()
            .register_type::<Mana>()
            .register_type::<Health>()
            .register_type::<Velocity>()
            .register_type::<Acceleration>()
            .register_type::<SpellChoices>()
            .register_type::<CursorPosition>()
            // Add a window for the SpellChoices resource.
            .add_plugins(ResourceInspectorPlugin::<SpellChoices>::default());
    }
}
