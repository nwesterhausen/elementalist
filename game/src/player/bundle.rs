use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use game_library::{enums::StatEnum, Health, Layer, Mana, MovementBundle, StatBundle, Xp};

/// Base stats for the player. These are the stats that the player starts with, and are used to
/// initiate the [`StatBundle`] for the player.
pub(super) const fn player_base_stats(stat: &StatEnum) -> f32 {
    match stat {
        StatEnum::MovementSpeed => 2.4,
        StatEnum::SpellRange => 50.0,
        StatEnum::Health => 10.0,
        StatEnum::Mana => 4.0,
        StatEnum::ManaRegeneration
        | StatEnum::ProjectileSpeed
        | StatEnum::ProjectileSize
        | StatEnum::ProjectileLifetime
        | StatEnum::MagicDamage => 1.0,
        StatEnum::HealthRegeneration => 0.05,
        StatEnum::DamageReduction
        | StatEnum::DamageResistance
        | StatEnum::DamageReflection
        | StatEnum::DamageAmplification
        | StatEnum::CriticalStrikeChance
        | StatEnum::CriticalStrikeDamage
        | StatEnum::LifeSteal
        | StatEnum::ManaSteal
        | StatEnum::StunResistance
        | StatEnum::DodgeChance
        | StatEnum::AttackDamage
        | StatEnum::AttackSpeed
        | StatEnum::AttackRange
        | StatEnum::PhysicalDamageReduction
        | StatEnum::PhysicalDamageResistance
        | StatEnum::PhysicalDamageReflection
        | StatEnum::PhysicalDamageAmplification
        | StatEnum::CooldownReduction
        | StatEnum::MagicalDamageReduction
        | StatEnum::MagicalDamageResistance
        | StatEnum::MagicalDamageReflection
        | StatEnum::MagicalDamageAmplification => 0.0,
    }
}

/// Base health for the player. Specified outside of `base_stats` because it's an integer.
pub(super) const BASE_HEALTH: u32 = 10;
/// Base mana for the player. Specified outside of `base_stats` because it's an integer.
pub(super) const BASE_MANA: u32 = 4;

/// Player is a marker component for the player. This is used to identify the player's entity
/// in queries and systems. This should be used instead of using the `PlayerAvatar` component
/// directly, as it allows for more flexibility in the future.
#[derive(Component, Debug, Reflect)]
pub struct Player;

/// `PlayerBundle` is a bundle of components that are used to create the player's entity. This is
/// used to spawn the player's entity in the game. They are a set of components that are integral
/// to the player and we want to ensure that they are always present when the player is spawned.
///
/// These are specifically related to spawning the [`PlayerAvatar`], and are used to track the
/// player's health, mana, stats, and experience points while in a game run. The overall player
/// skills and unlocked spells are tracked in another resource (`todo!()`).
#[derive(Bundle)]
pub struct PlayerBundle {
    /// The player's movement bundle.
    pub movement: MovementBundle,
    /// The player's sprite bundle.
    pub sprite: SpriteSheetBundle,
    /// The player's health.
    pub health: Health,
    /// The player's mana.
    pub mana: Mana,
    /// The player's stats.
    pub stats: StatBundle,
    /// The player's experience points.
    pub xp: Xp,
    /// Player marker component.
    pub player: Player,
    /// Player controller marker component (rapier)
    pub kinematic_controller: KinematicCharacterController,
    /// Physics collider
    pub collider: Collider,
    /// Physics rigid body
    pub rigid_body: RigidBody,
    /// 2d Layer position
    pub layer: Layer,
}
