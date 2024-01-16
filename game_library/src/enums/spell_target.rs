/// The target of a spell; where the spell can be targeted at
#[derive(Debug, Clone, Hash, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SpellTarget {
    /// The spell can be cast on any entity.
    Any,
    /// The spell can only be cast on the player.
    Player,
    /// The spell can only be cast on enemies.
    Enemy,
    /// The spell can only be cast on allies.
    Ally,
    /// No-one. The spell is targeted at the ground or other non-character.
    #[default]
    None,
}
