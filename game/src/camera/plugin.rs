use bevy::prelude::*;

use super::creation;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, creation::setup_camera);
    }
}
