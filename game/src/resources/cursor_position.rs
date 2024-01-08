use bevy::{prelude::*, window::PrimaryWindow};

/// Tracks the cursor position in the primary window
#[derive(Resource, Debug, Clone, Copy)]
pub struct OffsetCursorPosition {
    pub x: f32,
    pub y: f32,
}

impl Default for OffsetCursorPosition {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

/// Updates the cursor position resource
pub fn update_cursor_position_resource(
    mut cursor: EventReader<CursorMoved>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
    mut cursor_resource: ResMut<OffsetCursorPosition>,
) {
    let Ok(window) = primary_query.get_single() else {
        return;
    };
    let mut cursor_position = match cursor.read().last() {
        Some(cursor) => cursor.position,
        // This won't update the cursor position when it is outside the window
        None => return,
    };

    // Convert cursor position to window coordinates
    cursor_position.x -= window.width() / 2.0;
    cursor_position.y -= window.height() / 2.0;

    // Invert the y axis ?
    cursor_position.y *= -1.0;

    // Update the cursor position resource
    cursor_resource.x = cursor_position.x;
    cursor_resource.y = cursor_position.y;
}
