use bevy::ecs::{component::Component, system::Resource};
use serde::{Deserialize, Serialize};

/// How the spell is targeted when casting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Resource, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CastType {
    /// The spell is cast instantly
    Instant,
    /// The spell must be maintained to be active
    Channel,
    /// The spell is cast over a period of time (there is a delay before the spell is cast)
    Cast,
}
