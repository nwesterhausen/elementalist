//! A resource that stores the player's spell choices.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::Spell;
use crate::InternalId;

/// A resource that stores the player's spell choices.
#[derive(Resource, Default, Debug, Reflect, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpellSelection {
    /// The spell to cast when the player presses the primary cast button.
    primary: Option<String>,
    /// The spell to cast when the player presses the secondary cast button.
    secondary: Option<String>,
    /// The spell to cast when the player presses the tertiary cast button.
    tertiary: Option<String>,
    /// The spell to cast when the player presses the ultimate cast button.
    ultimate: Option<String>,
}

impl SpellSelection {
    /// Create a new spell selection with no spells chosen.
    #[must_use]
    pub fn new() -> Self {
        Self {
            primary: None,
            secondary: None,
            tertiary: None,
            ultimate: None,
        }
    }
    /// Set the primary spell choice at creation.
    #[must_use]
    pub fn with_primary(&self, spell: &Spell) -> Self {
        Self {
            primary: Some(spell.get_internal_id()),
            ..self.clone()
        }
    }
    /// Set the primary spell choice at creation using the spell's unique id.
    #[must_use]
    pub fn with_primary_by_id(&self, unique_id: String) -> Self {
        Self {
            primary: Some(unique_id),
            ..self.clone()
        }
    }
    /// Set the secondary spell choice at creation.
    #[must_use]
    pub fn with_secondary(&self, spell: &Spell) -> Self {
        Self {
            secondary: Some(spell.get_internal_id()),
            ..self.clone()
        }
    }
    /// Set the secondary spell choice at creation using the spell's unique id.
    #[must_use]
    pub fn with_secondary_by_id(&self, unique_id: String) -> Self {
        Self {
            secondary: Some(unique_id),
            ..self.clone()
        }
    }
    /// Set the tertiary spell choice at creation.
    #[must_use]
    pub fn with_tertiary(&self, spell: &Spell) -> Self {
        Self {
            tertiary: Some(spell.get_internal_id()),
            ..self.clone()
        }
    }
    /// Set the tertiary spell choice at creation using the spell's unique id.
    #[must_use]
    pub fn with_tertiary_by_id(&self, unique_id: String) -> Self {
        Self {
            tertiary: Some(unique_id),
            ..self.clone()
        }
    }

    /// Set the primary spell choice.
    pub fn set_primary(&mut self, spell: &Spell) {
        self.primary = Some(spell.get_internal_id());
    }
    /// Set the primary spell choice by spell id.
    pub fn set_primary_by_id(&mut self, unique_id: String) {
        self.primary = Some(unique_id);
    }
    /// Set the secondary spell choice.
    pub fn set_secondary(&mut self, spell: &Spell) {
        self.secondary = Some(spell.get_internal_id());
    }
    /// Set the secondary spell choice by spell id.
    pub fn set_secondary_by_id(&mut self, unique_id: String) {
        self.secondary = Some(unique_id);
    }
    /// Set the tertiary spell choice.
    pub fn set_tertiary(&mut self, spell: &Spell) {
        self.tertiary = Some(spell.get_internal_id());
    }
    /// Set the tertiary spell choice by spell id.
    pub fn set_tertiary_by_id(&mut self, unique_id: String) {
        self.tertiary = Some(unique_id);
    }
    /// Set the ultimate spell choice.
    pub fn set_ultimate(&mut self, spell: &Spell) {
        self.ultimate = Some(spell.get_internal_id());
    }
    /// Set the ultimate spell choice by spell id.
    pub fn set_ultimate_by_id(&mut self, unique_id: String) {
        self.ultimate = Some(unique_id);
    }

    /// Clear the primary spell choice.
    pub fn clear_primary(&mut self) {
        self.primary = None;
    }
    /// Clear the secondary spell choice.
    pub fn clear_secondary(&mut self) {
        self.secondary = None;
    }
    /// Clear the defensive spell choice.
    pub fn clear_tertiary(&mut self) {
        self.tertiary = None;
    }
    /// Clear the ultimate spell choice.
    pub fn clear_ultimate(&mut self) {
        self.ultimate = None;
    }
}
