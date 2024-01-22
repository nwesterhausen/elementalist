use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{enums::CastSlot, SpellData};

/// A resource that stores the player's spell choices.
#[derive(
    Resource,
    Default,
    Debug,
    Reflect,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    InspectorOptions,
)]
#[reflect(InspectorOptions)]
pub struct SpellChoices {
    /// The spell to cast when the player presses the primary cast button.
    pub primary: Option<String>,
    /// The spell to cast when the player presses the secondary cast button.
    pub secondary: Option<String>,
    /// The spell to cast when the player presses the defensive cast button.
    pub defensive: Option<String>,
    /// The spell to cast when the player presses the ultimate cast button.
    pub ultimate: Option<String>,
}

impl SpellChoices {
    /// Set the primary spell choice.
    pub fn set_primary(&mut self, spell: &SpellData) {
        if spell.cast_slot == CastSlot::Primary {
            self.primary = Some(spell.get_internal_id());
        }
    }
    /// Set the secondary spell choice.
    pub fn set_secondary(&mut self, spell: &SpellData) {
        if spell.cast_slot == CastSlot::Secondary {
            self.secondary = Some(spell.get_internal_id());
        }
    }
    /// Set the defensive spell choice.
    pub fn set_defensive(&mut self, spell: &SpellData) {
        if spell.cast_slot == CastSlot::Defensive {
            self.defensive = Some(spell.get_internal_id());
        }
    }
    /// Set the ultimate spell choice.
    pub fn set_ultimate(&mut self, spell: &SpellData) {
        if spell.cast_slot == CastSlot::Ultimate {
            self.ultimate = Some(spell.get_internal_id());
        }
    }
    /// Set the primary spell choice by spell id.
    pub fn set_primary_by_id(&mut self, spell_id: String) {
        self.primary = Some(spell_id);
    }
    /// Set the secondary spell choice by spell id.
    pub fn set_secondary_by_id(&mut self, spell_id: String) {
        self.secondary = Some(spell_id);
    }
    /// Set the defensive spell choice by spell id.
    pub fn set_defensive_by_id(&mut self, spell_id: String) {
        self.defensive = Some(spell_id);
    }
    /// Set the ultimate spell choice by spell id.
    pub fn set_ultimate_by_id(&mut self, spell_id: String) {
        self.ultimate = Some(spell_id);
    }
}
