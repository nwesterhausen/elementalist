use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::{Acceleration, Velocity};

/// Bundle that contains all components needed for moving things
#[derive(Bundle, Debug, Default, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct MovementBundle {
    /// The velocity component (how fast the entity is moving)
    pub velocity: Velocity,
    /// The acceleration component (how fast the entity is accelerating; i.e. changing velocity)
    pub acceleration: Acceleration,
}
