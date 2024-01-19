use crate::{enums::SpellTarget, enums::StatEnum, StatBonus};

/// Effects from a spell or other item.
///
/// Describes in detail how a this effect can be applied.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatEffect {
    /// An identifier for the effect.
    pub id: String,
    /// The name of the effect.
    pub name: String,
    /// The description of the effect.
    pub description: String,

    /// The base duration of the effect.
    ///
    /// This is measured in centiseconds.
    pub duration: u32,
    /// The maximum number of times this effect can be applied to a single target.
    ///
    /// A value of `0` means that there is no limit.
    pub max_stacks: u32,
    /// Which entities this effect can be applied to. If the choice here is [`SpellTarget::Any`],
    /// then the effect can affect any entity, as long as they are "hit" by the spell.
    pub target: SpellTarget,

    /// The stat effects of this spell.
    pub effects: Vec<(StatEnum, StatBonus)>,
}
