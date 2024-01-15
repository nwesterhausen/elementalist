/// Possible stats that a character can have. These are available for both the player
/// and the enemies.
#[derive(Debug, Clone, Hash, PartialEq, serde::Serialize, serde::Deserialize)]
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
