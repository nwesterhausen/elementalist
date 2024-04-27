use bevy::prelude::*;

use elementalist_game_library::CameraScaleLevel;

#[derive(Component)]
pub struct MainCamera;

/// Spawns a basic camera
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera,
        CameraScaleLevel::default(),
    ));
}

/// Zooms the camera based on the current scale level
pub fn zoom_camera(
    mut query: Query<(&mut OrthographicProjection, &CameraScaleLevel), With<MainCamera>>,
) {
    for (mut projection, scale_level) in &mut query {
        projection.scale = scale_level.value();
    }
}
