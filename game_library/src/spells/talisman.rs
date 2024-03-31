//! The spell talisman is a component that defines how the spell behaves.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::data_loader::storage::GameData;

/// The spell talisman defines how the spell behaves. It contains a shaping, a behavior, and a tier.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect, Component, Resource,
)]
#[allow(clippy::module_name_repetitions)]
pub struct SpellTalisman {
    /// The shaping of the spell talisman. This defines the shape of the spell.
    shaping: Shaping,
    /// The behavior of the spell talisman. This defines the behavior of the spell.
    behavior: Behavior,
    /// The tier of the spell talisman. This defines the power level of the spell.
    tier: Tier,
}

/// The shaping of the spell talisman. This defines the shape of the spell.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect, Component, Resource,
)]
pub enum Shaping {
    /// A projectile spell is a spell that launches away from the caster.
    Projectile,
    /// Area of Effect spells affect a specified area.
    AreaOfEffect,
    /// A cone spell affects an area in front of the caster in a cone shape.
    Cone,
    /// A line spell affects an area in front of the caster in a line.
    Line,
    /// A touch spell affects a target that the caster touches.
    Touch,
    /// A spell with the `OnSelf` shaping affects the caster.
    OnSelf,
    /// A spell with the `Melee` shaping acts as a melee attack.
    Melee,
}

impl std::fmt::Display for Shaping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Projectile => write!(f, "Projectile"),
            Self::AreaOfEffect => write!(f, "Area of Effect"),
            Self::Cone => write!(f, "Cone"),
            Self::Line => write!(f, "Line"),
            Self::Touch => write!(f, "Touch"),
            Self::OnSelf => write!(f, "Self"),
            Self::Melee => write!(f, "Melee"),
        }
    }
}

impl Shaping {
    /// Get the index of the shaping in the engraving tileset.
    #[must_use]
    pub const fn engraving_index(&self) -> usize {
        match self {
            Self::Projectile => 0,
            Self::AreaOfEffect => 1,
            Self::Cone => 2,
            Self::Line => 3,
            Self::Touch => 4,
            Self::OnSelf => 6,
            Self::Melee => 5,
        }
    }
}

/// The behavior of the spell talisman. This defines the behavior of the spell.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect, Component, Resource,
)]
pub enum Behavior {
    /// A spell with the `Damage` behavior deals damage to the target.
    Damage,
    /// A spell with the `Heal` behavior heals the target.
    Heal,
    /// A spell with the `Buff` behavior applies a buff to the target.
    Buff,
    /// A spell with the `Debuff` behavior applies a debuff to the target.
    Debuff,
    /// A spell with the `Utility` behavior has a utility effect.
    Utility,
    /// A spell with the `Summon` behavior summons a creature.
    Summon,
    /// A spell with the `Polymorph` behavior transforms the target.
    Polymorph,
    /// A spell with the `Teleport` behavior teleports the target.
    Teleport,
}

impl std::fmt::Display for Behavior {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Damage => write!(f, "Damage"),
            Self::Heal => write!(f, "Heal"),
            Self::Buff => write!(f, "Buff"),
            Self::Debuff => write!(f, "Debuff"),
            Self::Utility => write!(f, "Utility"),
            Self::Summon => write!(f, "Summon"),
            Self::Polymorph => write!(f, "Polymorph"),
            Self::Teleport => write!(f, "Teleport"),
        }
    }
}

impl Behavior {
    /// Get the index of the behavior in the engraving tileset.
    #[must_use]
    pub const fn engraving_index(&self) -> usize {
        match self {
            Self::Damage => 7,
            Self::Heal => 8,
            Self::Buff => 9,
            Self::Debuff => 10,
            Self::Utility => 11,
            Self::Summon => 12,
            Self::Polymorph => 13,
            Self::Teleport => 14,
        }
    }
}

/// The tier of the spell talisman. This defines the power level of the spell.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect, Component, Resource,
)]
pub enum Tier {
    /// A spell with the `Mundane` tier is the lowest tier.
    Mundane,
    /// A spell with the `Common` tier is the second lowest tier.
    Common,
    /// A spell with the `Uncommon` tier is the third lowest tier.
    Uncommon,
    /// A spell with the `Rare` tier is the fourth lowest tier.
    Rare,
    /// A spell with the `Epic` tier is the fifth lowest tier.
    Epic,
    /// A spell with the `Legendary` tier is the third highest tier.
    Legendary,
    /// A spell with the `Mythic` tier is the second highest tier.
    Mythic,
    /// A spell with the `Divine` tier is the highest tier.
    Divine,
    /// `Astral` is a special tier that is above Divine. It cannot be inscribed on a spell talisman normally.
    Astral,
    /// `Unique` should only be used for restricted spells that are unique to a specific character or have other requirements.
    /// It cannot be inscribed on a spell talisman normally.
    Unique,
}

impl std::fmt::Display for Tier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mundane => write!(f, "Mundane"),
            Self::Common => write!(f, "Common"),
            Self::Uncommon => write!(f, "Uncommon"),
            Self::Rare => write!(f, "Rare"),
            Self::Epic => write!(f, "Epic"),
            Self::Legendary => write!(f, "Legendary"),
            Self::Mythic => write!(f, "Mythic"),
            Self::Divine => write!(f, "Divine"),
            Self::Astral => write!(f, "Astral"),
            Self::Unique => write!(f, "Unique"),
        }
    }
}

impl Tier {
    /// Get the index of the talisman for this tier.
    #[must_use]
    pub const fn talisman_index(&self) -> usize {
        match self {
            Self::Mundane => 9,
            Self::Common => 8,
            Self::Uncommon => 7,
            Self::Rare => 6,
            Self::Epic => 5,
            Self::Legendary => 4,
            Self::Mythic => 3,
            Self::Divine => 2,
            Self::Astral => 1,
            Self::Unique => 0,
        }
    }
}

impl Default for SpellTalisman {
    fn default() -> Self {
        Self {
            shaping: Shaping::Projectile,
            behavior: Behavior::Damage,
            tier: Tier::Mundane,
        }
    }
}

/// Spell talisman ui nodes
#[derive(Debug)]
pub struct SpellTalismanUi {
    /// Base node bundle
    base: NodeBundle,
    /// The background talisman image
    background: AtlasImageBundle,
    /// The shaping engraving
    shaping: AtlasImageBundle,
    /// The behavior engraving
    behavior: AtlasImageBundle,
}

impl SpellTalisman {
    /// Talisman sprite sheet identifier.
    pub const SPRITE_SHEET: &'static str = "talismans";
    /// Engraving sprite sheet identifier.
    pub const ENGRAVING_SHEET: &'static str = "talisman_engraving";

    /// Set the shaping of the spell talisman.
    #[must_use]
    pub const fn with_shaping(mut self, shaping: Shaping) -> Self {
        self.shaping = shaping;
        self
    }

    /// Set the behavior of the spell talisman.
    #[must_use]
    pub const fn with_behavior(mut self, behavior: Behavior) -> Self {
        self.behavior = behavior;
        self
    }

    /// Set the tier of the spell talisman.
    #[must_use]
    pub const fn with_tier(mut self, tier: Tier) -> Self {
        self.tier = tier;
        self
    }

    /// Create a new spell talisman with the given shaping, behavior, and tier.
    #[must_use]
    pub const fn new(tier: Tier, shaping: Shaping, behavior: Behavior) -> Self {
        Self {
            shaping,
            behavior,
            tier,
        }
    }

    /// Get the shaping of the spell talisman.
    #[must_use]
    pub const fn shaping(&self) -> Shaping {
        self.shaping
    }

    /// Get the behavior of the spell talisman.
    #[must_use]
    pub const fn behavior(&self) -> Behavior {
        self.behavior
    }

    /// Get the tier of the spell talisman.
    #[must_use]
    pub const fn tier(&self) -> Tier {
        self.tier
    }

    /// Get a UI Node drawing this talisman.
    ///
    /// The node will draw the talisman sprite and engravings.
    #[must_use]
    pub fn ui_nodes(&self, game_data: GameData) -> Option<SpellTalismanUi> {
        let mut node = NodeBundle {
            background_color: Color::NONE,
            style: Style {
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                ..default()
            },
            ..default()
        };

        // need to get the `AtlasImageBundle` for the talisman sprite
        let Some(talisman_tileset) = game_data.tile_atlas.get(Self::SPRITE_SHEET) else {
            error!("Failed to get talisman tileset");
            return None;
        };
        let talisman_index = self.tier.talisman_index();
        let talisman_atlas_sprite = AtlasImageBundle {
            texture_atlas: TextureAtlas {
                index: talisman_index,
                layout: talisman_tileset.atlas_handle.clone(),
            },
            image: UiImage {
                texture: talisman_tileset.texture_handle.clone(),
                ..default()
            },
            ..default()
        };

        // get the engraving sprite
        let Some(engraving_tileset) = game_data.tile_atlas.get(Self::ENGRAVING_SHEET) else {
            error!("Failed to get engraving tileset");
            return None;
        };
        let shaping_index = self.shaping as usize;
        let behavior_index = self.behavior as usize;
        let shaping_atlas_sprite = AtlasImageBundle {
            texture_atlas: TextureAtlas {
                index: shaping_index,
                layout: engraving_tileset.atlas_handle.clone(),
            },
            image: UiImage {
                texture: engraving_tileset.texture_handle.clone(),
                ..default()
            },
            ..default()
        };
        let behavior_atlas_sprite = AtlasImageBundle {
            texture_atlas: TextureAtlas {
                index: behavior_index,
                layout: engraving_tileset.atlas_handle.clone(),
            },
            image: UiImage {
                texture: engraving_tileset.texture_handle.clone(),
                ..default()
            },
            ..default()
        };

        Some(SpellTalismanUi {
            base: node,
            background: talisman_atlas_sprite,
            shaping: shaping_atlas_sprite,
            behavior: behavior_atlas_sprite,
        })
    }
}

impl std::fmt::Display for SpellTalisman {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} Talisman of {} {}",
            self.tier, self.shaping, self.behavior
        )
    }
}
