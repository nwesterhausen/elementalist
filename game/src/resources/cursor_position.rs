use bevy::{prelude::*, window::PrimaryWindow};

use crate::camera::MainCamera;

#[derive(Resource, Debug, Copy, Clone, Default)]
pub struct CursorPosition {
    pub position: Vec2,
}

pub fn update_cursor_position(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    let Ok(window) = windows.get_single() else {
        tracing::error!("cursor_position: No primary window found");
        return;
    };
    let Ok((camera, camera_transform)) = camera_q.get_single() else {
        tracing::error!("cursor_position: No main camera found");
        return;
    };

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        cursor_position.position = world_position;
    }
}
