use bevy::ecs::{component::Component, system::Resource};
use serde::{Deserialize, Serialize};

/// The type of collision the spell has.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component, Resource, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub enum SpellCollision {
    /// The spell has no collision.
    #[default]
    None,
    /// The spell has a point collision. (e.g. a stone dart or similar projectile)
    Point,
    /// The spell has a line collision. (e.g. a lightning bolt or vine whip)
    Line,
    /// The spell has a cone collision. (e.g. cone of frost)
    Cone,
    /// The spell has a circle collision. (e.g. a fireball)
    Circle,
    /// The spell has a rectangle collision. (e.g. a wall of fire)
    Rectangle,
}
