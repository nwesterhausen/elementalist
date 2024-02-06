use bevy::reflect::Reflect;

/// The attachment point for a particle effect.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Reflect, PartialEq, Eq, Hash)]
pub enum ParticleAttachment {
    /// The particle is emitted once on the caster when the spell is cast.
    Cast,
    /// The particle is emitted from the projectile while it is in flight.
    Projectile,
    /// The particle is emitted when the projectile hits something.
    Impact,
    /// The particle is emitted on the target when the spell hits.
    Target,
    /// The particle is emitted on the caster when the spell is cast.
    Caster,
    /// The particle remains on the ground where the spell impacts.
    Ground,
    /// The particle is emitted from the summoned entity.
    Summon,
}
