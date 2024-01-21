//! A resource that stores the cursor position in world coordinates.

use bevy::{prelude::*, reflect::Reflect};
use bevy_inspector_egui::prelude::*;

/// A resource that stores the cursor position in world coordinates.
#[derive(Resource, Debug, Copy, Clone, Default, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct CursorPosition {
    /// The position of the cursor
    pub position: Vec2,
}
