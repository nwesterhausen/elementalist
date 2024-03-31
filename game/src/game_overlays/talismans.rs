//! Draw some talismans

use bevy::prelude::*;
use game_library::spells::talisman::{Behavior, Shaping, SpellTalisman, Tier};

const SAMPLE_TALISMANS: Vec<SpellTalisman> = vec![
    SpellTalisman::new(Tier::Mundane, Shaping::Cone, Behavior::Debuff),
    SpellTalisman::new(Tier::Rare, Shaping::Projectile, Behavior::Damage),
    SpellTalisman::new(Tier::Legendary, Shaping::Area, Behavior::Heal),
    SpellTalisman::new(Tier::Uncommon, Shaping::Line, Behavior::Damage),
    SpellTalisman::new(Tier::Common, Shaping::Touch, Behavior::Debuff),
    SpellTalisman::new(Tier::Epic, Shaping::Area, Behavior::Utility),
    SpellTalisman::new(Tier::Divine, Shaping::OnSelf, Behavior::Polymorph),
    SpellTalisman::new(Tier::Astral, Shaping::Melee, Behavior::Teleport),
    SpellTalisman::new(Tier::Unique, Shaping::Area, Behavior::Summon),
];
