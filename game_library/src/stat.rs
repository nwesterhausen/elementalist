use bevy::{
    ecs::{component::Component, system::Resource},
    reflect::Reflect,
};
use bevy_inspector_egui::inspector_options::{InspectorOptions, ReflectInspectorOptions};
use serde::{Deserialize, Serialize};

use crate::StatBonus;

/// A stat is a value that should be used to represent a character's speed,
/// strength or other statistic. This is not an [`game_library::Attribute`] which has a
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
/// - other items in [`game_library::enums::StatEnum`]
#[derive(
    Resource, Component, Reflect, Clone, Copy, PartialEq, Serialize, Deserialize, InspectorOptions,
)]
#[reflect(InspectorOptions)]
pub struct Stat {
    /// The base value of the stat. This is the value that the stat will
    /// scale from.
    #[inspector(speed = 0.1)]
    base_value: f32,
    /// The current value of the stat. This is the value that will be
    /// used for calculations.
    #[inspector(speed = 0.1)]
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
    /// Creates a new stat bundle with the given base value. This sets the initial
    /// bonus value to its default of 1.0.
    ///
    /// # Arguments
    ///
    /// * `base_value` - The base value of the stat.
    ///
    /// # Returns
    ///
    /// * A new stat bundle with the given base value.
    ///
    /// # Examples
    ///
    /// ```
    /// use game_library::Stat;
    ///
    /// let stat = Stat::new(10.0);
    /// assert_eq!(stat.value(), 10.0);
    /// assert_eq!(stat.base_value(), 10.0);
    /// assert_eq!(stat.bonus(), 1.0);
    /// ```
    #[must_use]
    pub fn new(base_value: f32) -> Self {
        Self {
            base_value,
            value: base_value,
            bonus: StatBonus::default(),
        }
    }

    /// Returns the current value of the stat.
    ///
    /// # Examples
    ///
    /// ```
    /// use game_library::Stat;
    ///
    /// let mut stat = Stat::new(6.0);
    /// assert_eq!(stat.value(), 6.0);
    /// stat.add_bonus(0.5);
    /// assert_eq!(stat.value(), 9.0);
    /// ```
    #[must_use]
    pub fn value(&self) -> f32 {
        self.value
    }

    /// Returns the current bonus of the stat.
    ///
    /// # Examples
    ///
    /// ```
    /// use game_library::Stat;
    ///
    /// let mut stat = Stat::new(10.0);
    /// assert_eq!(stat.bonus(), 1.0);
    /// stat.add_bonus(0.5);
    /// assert_eq!(stat.bonus(), 1.5);
    /// ```
    #[must_use]
    pub fn bonus(&self) -> f32 {
        self.bonus.value()
    }

    /// Returns the base value of the stat.
    ///
    /// # Examples
    ///
    /// ```
    /// use game_library::Stat;
    ///
    /// let mut stat = Stat::new(6.0);
    /// assert_eq!(stat.base_value(), 6.0);
    /// stat.add_bonus(0.5);
    /// assert_eq!(stat.base_value(), 6.0);
    /// stat.add_base_value(1.0);
    /// assert_eq!(stat.base_value(), 7.0);
    /// ```
    #[must_use]
    pub fn base_value(&self) -> f32 {
        self.base_value
    }

    /// Updates the value of the stat.
    ///
    /// This is called automatically when the base value or bonus is changed.
    fn update_value(&mut self) {
        self.value = self.base_value * self.bonus.value();
    }

    /// Add to the base value of the stat. The base value is one part of what
    /// makes up the final value of the stat. The other part is the bonus.
    ///
    /// This will also update the value of the stat. (Apply the bonus to the
    /// new base value, updating what `value()` returns.)
    ///
    /// # Arguments
    ///
    /// * `value` - The value to add to the base value.
    ///
    /// # Examples
    ///
    /// ```
    /// use game_library::Stat;
    ///
    /// let mut stat = Stat::new(6.0);
    /// assert_eq!(stat.base_value(), 6.0);
    /// assert_eq!(stat.value(), 6.0);
    /// stat.add_base_value(1.0);
    /// assert_eq!(stat.base_value(), 7.0);
    /// assert_eq!(stat.value(), 7.0);
    /// stat.add_bonus(0.5);
    /// assert_eq!(stat.base_value(), 7.0);
    /// assert_eq!(stat.value(), 10.5);
    /// // Now when we add one to the base value, we can see `value()` is updated against the new bonus.
    /// stat.add_base_value(1.0);
    /// assert_eq!(stat.base_value(), 8.0);
    /// assert_eq!(stat.value(), 12.0);
    /// ```
    pub fn add_base_value(&mut self, value: f32) {
        self.base_value += value;
        self.update_value();
    }

    /// Subtract from the base value of the stat.
    ///
    /// This will also update the value of the stat. (Apply the bonus to the
    /// new base value, updating what `value()` returns.)
    ///
    /// # Arguments
    ///
    /// * `value` - The value to subtract from the base value.
    pub fn subtract_base_value(&mut self, value: f32) {
        self.base_value -= value;
        self.update_value();
    }

    /// Multiply the base value of the stat.
    ///
    /// This will also update the value of the stat. (Apply the bonus to the
    /// new base value, updating what `value()` returns.)
    ///
    /// # Arguments
    ///
    /// * `value` - The value to multiply the base value by.
    pub fn multiply_base_value(&mut self, value: f32) {
        self.base_value *= value;
        self.update_value();
    }

    /// Divide the base value of the stat.
    ///
    /// This will also update the value of the stat. (Apply the bonus to the
    /// new base value, updating what `value()` returns.)
    ///
    /// # Arguments
    ///
    /// * `value` - The value to divide the base value by.
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
    ///
    /// This will also update the value of the stat. (Apply the bonus to the
    /// new base value, updating what `value()` returns.)
    ///
    /// # Arguments
    ///
    /// * `value` - The value to add to the bonus.
    ///
    /// See [`StatBonus::add_value`] for more information.
    pub fn add_bonus(&mut self, value: f32) {
        self.bonus.add_value(value);
        self.update_value();
    }

    /// Subtract from the bonus of the stat.
    ///
    /// This will also update the value of the stat. (Apply the bonus to the
    /// new base value, updating what `value()` returns.)
    ///
    /// # Arguments
    ///
    /// * `value` - The value to subtract from the bonus.
    ///
    /// See [`StatBonus::subtract_value`] for more information.
    pub fn subtract_bonus(&mut self, value: f32) {
        self.bonus.subtract_value(value);
        self.update_value();
    }

    /// Multiply the bonus of the stat.
    ///
    /// This will also update the value of the stat. (Apply the bonus to the
    /// new base value, updating what `value()` returns.)
    ///
    /// # Arguments
    ///
    /// * `value` - The value to multiply the bonus by.
    ///
    /// See [`StatBonus::multiply_value`] for more information.
    pub fn multiply_bonus(&mut self, value: f32) {
        self.bonus.multiply_value(value);
        self.update_value();
    }

    /// Divide the bonus of the stat.
    ///
    /// This will also update the value of the stat. (Apply the bonus to the
    /// new base value, updating what `value()` returns.)
    ///
    /// # Arguments
    ///
    /// * `value` - The value to divide the bonus by.
    ///
    /// See [`StatBonus::divide_value`] for more information.
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

    /// Set the base value of the stat.
    ///
    /// This will overwrite the existing base value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set the base value to.
    ///
    /// # Examples
    ///
    /// ```
    /// use game_library::Stat;
    ///
    /// let mut stat = Stat::new(10.0);
    /// assert_eq!(stat.base_value(), 10.0);
    /// assert_eq!(stat.value(), 10.0);
    /// stat.set_base_value(5.0);
    /// assert_eq!(stat.base_value(), 5.0);
    /// assert_eq!(stat.value(), 5.0);
    /// ```
    pub fn set_base_value(&mut self, value: f32) {
        self.base_value = value;
        self.update_value();
    }

    /// Set the bonus of the stat.
    ///
    /// This will overwrite the existing bonus.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set the bonus to.
    ///
    /// # Examples
    ///
    /// ```
    /// use game_library::Stat;
    ///
    /// let mut stat = Stat::new(10.0);
    /// assert_eq!(stat.bonus(), 1.0);
    /// assert_eq!(stat.value(), 10.0);
    /// stat.set_bonus_value(5.0);
    /// assert_eq!(stat.bonus(), 5.0);
    /// assert_eq!(stat.value(), 50.0);
    /// ```
    pub fn set_bonus_value(&mut self, value: f32) {
        self.bonus.set_value(value);
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
