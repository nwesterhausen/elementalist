pub enum SpellTarget {
    /// The spell can be cast on any entity.
    Any,
    /// The spell can only be cast on the player.
    Player,
    /// The spell can only be cast on enemies.
    Enemy,
    /// The spell can only be cast on allies.
    Ally,
}
