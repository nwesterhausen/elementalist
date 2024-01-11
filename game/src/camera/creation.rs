use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

/// Spawns a basic camera
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

// /// Zooms in on the camera (how to do this for future reference)
// pub fn zoom_in(mut query: Query<&mut OrthographicProjection, With<Camera>>, time: Res<Time>) {
//     for mut projection in query.iter_mut() {
//         projection.scale -= 0.1 * time.delta_seconds();

//         println!("Current zoom scale: {}", projection.scale);
//     }
// }
