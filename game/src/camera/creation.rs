use bevy::prelude::*;

/// Spawns a basic camera
pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
