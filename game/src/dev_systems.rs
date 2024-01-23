use bevy::prelude::*;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use game_library::{
    enums::Skill,
    font_resource::{FontChoice, FontResource},
    Acceleration, Attribute, CameraScaleLevel, CursorPosition, Health, Mana, MovementBundle,
    SpellChoices, Stat, StatBonus, Velocity, Volume,
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
            .register_type::<CameraScaleLevel>()
            .register_type::<MovementBundle>()
            .register_type::<FontResource>()
            .register_type::<FontChoice>()
            .add_plugins(
                // Add a window for the SpellChoices resource.
                ResourceInspectorPlugin::<SpellChoices>::default(),
            );
    }
}
