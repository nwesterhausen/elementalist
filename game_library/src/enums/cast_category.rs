use bevy::{
    ecs::{component::Component, system::Resource},
    reflect::Reflect,
};
use serde::{Deserialize, Serialize};

/// How the spell is targeted when casting.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Resource, Serialize, Deserialize, Reflect,
)]
#[serde(rename_all = "camelCase")]
pub enum CastCategory {
    /// The spell is cast by physically touching the target.
    Touch,
    /// The spell is cast by targeting another entity.
    Projectile,
    /// The spell is cast by targeting an area on the ground.
    Ground,
    /// The spell is cast on the caster.
    OnSelf,
    /// The spell is cast by summoning a minion (or other entity)
    Summon,
}
