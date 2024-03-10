use bevy::prelude::*;

use game_library::settings::{
    AccessibilitySettings, GameplaySettings, VideoSettings, VolumeSettings,
};
use game_library::Xp;
use game_library::{
    enums::Skill,
    font_resource::{FontChoice, FontResource},
    Acceleration, Attribute, CameraScaleLevel, CursorPosition, Health, Mana, MovementBundle, Stat,
    StatBonus, Volume,
};

pub struct DevSystemsPlugin;

impl Plugin for DevSystemsPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app.register_type::<Attribute>()
            .register_type::<Skill>()
            .register_type::<Stat>()
            .register_type::<StatBonus>()
            .register_type::<Volume>()
            .register_type::<Mana>()
            .register_type::<Health>()
            .register_type::<Xp>()
            .register_type::<Acceleration>()
            .register_type::<CursorPosition>()
            .register_type::<CameraScaleLevel>()
            .register_type::<MovementBundle>()
            .register_type::<FontResource>()
            .register_type::<FontChoice>()
            .register_type::<VolumeSettings>()
            .register_type::<VideoSettings>()
            .register_type::<GameplaySettings>()
            .register_type::<AccessibilitySettings>();
    }
}
