use bevy::reflect::Reflect;
use bevy_inspector_egui::inspector_options::{InspectorOptions, ReflectInspectorOptions};

/// Possible stats that a character can have. These are available for both the player
/// and the enemies.
#[derive(
    Debug,
    Clone,
    Hash,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    Reflect,
    InspectorOptions,
)]
#[reflect(InspectorOptions)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
pub enum StatEnum {
    /// The maximum health of the character. (Health is measured as an integer.)
    Health,
    /// The maximum mana of the character. (Mana is measured as an integer.)
    Mana,
    /// Reduce the amount of any incoming damage (one point here is one point of damage)
    DamageReduction,
    /// Resist a percentage of any incoming damage (after damage reduction).
    DamageResistance,
    /// Reflect a percentage of any incoming damage.
    DamageReflection,
    /// Amplify the amount of incoming damage (after damage reduction and resistance).
    DamageAmplification,
    /// A chance to critical strike with any attack
    CriticalStrikeChance,
    /// The damage amount of a critical strike is scaled by this value.
    CriticalStrikeDamage,
    /// Amount of health that is restored when a character deals damage.
    ///
    /// This is measured as a percentage of the damage dealt.
    LifeSteal,
    /// How much mana is restored when a character deals damage.
    ///
    /// Note: This is probably just a flat amount of mana restored.
    ManaSteal,
    /// How fast an entity moves, measured in centimeters.
    MovementSpeed,
    /// Reduces the duration of a stun by a percentage.
    StunResistance,
    /// Passive health regeneration.
    HealthRegeneration,
    /// Passive mana regeneration.
    ManaRegeneration,
    /// Projectile speed modifier.
    ProjectileSpeed,
    /// Projectile size modifier.
    ProjectileSize,
    /// Projectile lifetime modifier.
    ProjectileLifetime,
    /// Chance to evade an attack/damage
    DodgeChance,

    // ### PHYSICAL STATS ###
    /// The amount of damage that a character does with a physical attack.
    AttackDamage,
    /// The cooldown modifier for physical attacks.
    AttackSpeed,
    /// The range modifier of a physical attack.
    AttackRange,
    /// A flat reduction of incoming physical damage.
    PhysicalDamageReduction,
    /// A percentage reduction of incoming physical damage.
    PhysicalDamageResistance,
    /// A percentage reflection of incoming physical damage.
    PhysicalDamageReflection,
    /// A percentage amplification of incoming physical damage.
    PhysicalDamageAmplification,

    // ### MAGICAL STATS ###
    /// The amount of damage that a character does with a magical attack.
    MagicDamage,
    /// The cooldown modifier for magical attacks.
    CooldownReduction,
    /// The range modifier of a magical attack.
    SpellRange,
    /// A flat reduction of incoming magical damage.
    MagicalDamageReduction,
    /// A percentage reduction of incoming magical damage.
    MagicalDamageResistance,
    /// A percentage reflection of incoming magical damage.
    MagicalDamageReflection,
    /// A percentage amplification of incoming magical damage.
    MagicalDamageAmplification,
}

impl std::fmt::Display for StatEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Health => write!(f, "Health"),
            Self::Mana => write!(f, "Mana"),
            Self::DamageReduction => write!(f, "Damage Reduction"),
            Self::DamageResistance => write!(f, "Damage Resistance"),
            Self::DamageReflection => write!(f, "Damage Reflection"),
            Self::DamageAmplification => write!(f, "Damage Amplification"),
            Self::CriticalStrikeChance => write!(f, "Critical Strike Chance"),
            Self::CriticalStrikeDamage => write!(f, "Critical Strike Damage"),
            Self::LifeSteal => write!(f, "Life Steal"),
            Self::ManaSteal => write!(f, "Mana Steal"),
            Self::MovementSpeed => write!(f, "Movement Speed"),
            Self::StunResistance => write!(f, "Stun Resistance"),
            Self::HealthRegeneration => write!(f, "Health Regeneration"),
            Self::ManaRegeneration => write!(f, "Mana Regeneration"),
            Self::ProjectileSpeed => write!(f, "Projectile Speed"),
            Self::ProjectileSize => write!(f, "Projectile Size"),
            Self::ProjectileLifetime => write!(f, "Projectile Lifetime"),
            Self::DodgeChance => write!(f, "Dodge Chance"),
            Self::AttackDamage => write!(f, "Attack Damage"),
            Self::AttackSpeed => write!(f, "Attack Speed"),
            Self::AttackRange => write!(f, "Attack Range"),
            Self::PhysicalDamageReduction => write!(f, "Physical Damage Reduction"),
            Self::PhysicalDamageResistance => write!(f, "Physical Damage Resistance"),
            Self::PhysicalDamageReflection => write!(f, "Physical Damage Reflection"),
            Self::PhysicalDamageAmplification => write!(f, "Physical Damage Amplification"),
            Self::MagicDamage => write!(f, "Magic Damage"),
            Self::CooldownReduction => write!(f, "Cooldown Reduction"),
            Self::SpellRange => write!(f, "Spell Range"),
            Self::MagicalDamageReduction => write!(f, "Magical Damage Reduction"),
            Self::MagicalDamageResistance => write!(f, "Magical Damage Resistance"),
            Self::MagicalDamageReflection => write!(f, "Magical Damage Reflection"),
            Self::MagicalDamageAmplification => write!(f, "Magical Damage Amplification"),
        }
    }
}
