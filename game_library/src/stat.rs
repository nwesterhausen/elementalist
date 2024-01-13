use bevy::{
    ecs::{component::Component, system::Resource},
    reflect::Reflect,
};
use serde::{Deserialize, Serialize};

use crate::StatBonus;

/// A stat is a value that should be used to represent a character's speed,
/// strength or other statistic. This is not an [`Attribute`] which has a
/// maximum and exists between 0 and that maximum. A stat is a value that
/// really is a value which can be used directly in the game.
///
/// Things you might use [`Stat`] for:
///
/// - speed
/// - defense
/// - base damage
/// - damage reduction
/// - attack speed
#[derive(Resource, Component, Reflect, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Stat {
    /// The base value of the stat. This is the value that the stat will
    /// scale from.
    base_value: f32,
    /// The current value of the stat. This is the value that will be
    /// used for calculations.
    value: f32,
    /// The current bonus applied the stat. This then multiplied to get
    /// the final value.
    bonus: StatBonus,
}

impl Default for Stat {
    fn default() -> Self {
        Self {
            base_value: 0.0,
            value: 0.0,
            bonus: StatBonus::default(),
        }
    }
}

impl Stat {
    /// Creates a new stat bundle with the given base value.
    pub fn new(base_value: f32) -> Self {
        Self {
            base_value,
            value: base_value,
            bonus: StatBonus::default(),
        }
    }

    /// Returns the current value of the stat.
    pub fn value(&self) -> f32 {
        self.value
    }

    /// Returns the current bonus of the stat.
    pub fn bonus(&self) -> f32 {
        self.bonus.value()
    }

    /// Returns the base value of the stat.
    pub fn base_value(&self) -> f32 {
        self.base_value
    }

    /// Updates the value of the stat.
    fn update_value(&mut self) {
        self.value = self.base_value * self.bonus.value();
    }

    /// Add to the base value of the stat.
    pub fn add_base_value(&mut self, value: f32) {
        self.base_value += value;
        self.update_value();
    }

    /// Subtract from the base value of the stat.
    pub fn subtract_base_value(&mut self, value: f32) {
        self.base_value -= value;
        self.update_value();
    }

    /// Multiply the base value of the stat.
    pub fn multiply_base_value(&mut self, value: f32) {
        self.base_value *= value;
        self.update_value();
    }

    /// Divide the base value of the stat.
    pub fn divide_base_value(&mut self, value: f32) {
        // Guard against divide by zero
        if value == 0.0 {
            self.base_value = 0.0;
            self.update_value();
            return;
        }
        // Use our existing multiply function to clamp
        self.multiply_base_value(1.0 / value);
    }

    /// Add to the bonus of the stat.
    pub fn add_bonus(&mut self, value: f32) {
        self.bonus.add_value(value);
        self.update_value();
    }

    /// Subtract from the bonus of the stat.
    pub fn subtract_bonus(&mut self, value: f32) {
        self.bonus.subtract_value(value);
        self.update_value();
    }

    /// Multiply the bonus of the stat.
    pub fn multiply_bonus(&mut self, value: f32) {
        self.bonus.multiply_value(value);
        self.update_value();
    }

    /// Divide the bonus of the stat.
    pub fn divide_bonus(&mut self, value: f32) {
        // Guard against divide by zero
        if value == 0.0 {
            self.bonus.set_value(0.0);
            self.update_value();
            return;
        }
        // Use our existing multiply function to clamp
        self.bonus.divide_value(value);
        self.update_value();
    }
}

impl From<f32> for Stat {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

impl std::fmt::Debug for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatBundle")
            .field("base_value", &self.base_value)
            .field("value", &self.value)
            .field("bonus", &self.bonus)
            .finish()
    }
}

impl std::fmt::Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({} * {})", self.value, self.base_value, self.bonus)
    }
}
