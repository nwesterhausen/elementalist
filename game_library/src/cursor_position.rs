//! A resource that stores the cursor position in world coordinates.
//!
//! # Example
//!
//! ```no_run
//! use bevy::prelude::*;
//! use bevy::window::PrimaryWindow;
//! use game_library::CursorPosition;
//!
//! #[derive(Component)]
//! struct MainCamera;
//!
//! /// System which runs on [Update] and updates the cursor position
//! pub fn update_cursor_position(
//!    windows: Query<&Window, With<PrimaryWindow>>,
//!    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
//!    mut cursor_position: ResMut<CursorPosition>,
//! ) {
//!    let Ok(window) = windows.get_single() else {
//!        tracing::error!("cursor_position: No primary window found");
//!        return;
//!    };
//!    let Ok((camera, camera_transform)) = camera_q.get_single() else {
//!        tracing::error!("cursor_position: No main camera found");
//!        return;
//!    };
//!
//!    if let Some(world_position) = window
//!        .cursor_position()
//!        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
//!    {
//!        cursor_position.position = world_position;
//!    }
//! }
//!
//! /// An example system which prints the cursor position to the console
//! pub fn print_cursor_position(cursor_position: Res<CursorPosition>) {
//!   println!("Cursor Position: {:?}", cursor_position.position);
//! }
//!```

use bevy::{prelude::*, reflect::Reflect};
use bevy_inspector_egui::prelude::*;

/// A resource that stores the cursor position in world coordinates.
#[derive(Resource, Debug, Copy, Clone, Default, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct CursorPosition {
    /// The position of the cursor
    pub position: Vec2,
}
