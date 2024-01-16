use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use game_library::{CastSlot, SpellData};
use serde::{Deserialize, Serialize};

#[derive(
    Resource, Default, Debug, Reflect, Clone, PartialEq, Serialize, Deserialize, InspectorOptions,
)]
#[reflect(InspectorOptions)]
pub struct SpellChoices {
    pub primary: Option<String>,
    pub secondary: Option<String>,
    pub defensive: Option<String>,
    pub ultimate: Option<String>,
}

impl SpellChoices {
    pub fn set_primary(&mut self, spell: SpellData) {
        if spell.cast_slot == CastSlot::Primary {
            self.primary = Some(spell.get_internal_id());
        }
    }
    pub fn set_secondary(&mut self, spell: SpellData) {
        if spell.cast_slot == CastSlot::Secondary {
            self.secondary = Some(spell.get_internal_id());
        }
    }
    pub fn set_defensive(&mut self, spell: SpellData) {
        if spell.cast_slot == CastSlot::Defensive {
            self.defensive = Some(spell.get_internal_id());
        }
    }
    pub fn set_ultimate(&mut self, spell: SpellData) {
        if spell.cast_slot == CastSlot::Ultimate {
            self.ultimate = Some(spell.get_internal_id());
        }
    }
}
