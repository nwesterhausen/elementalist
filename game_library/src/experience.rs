use std::ops::AddAssign;

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use serde::{Deserialize, Serialize};

/// Experience component
///
/// Tracks the amount of xp accumulated by an entity. The xp curve is defined by the following
/// formula:
///
/// `(level - 1) ^ (factor_a / factor_b) / factor_c + base_xp`
///
/// The result is rounded down to the nearest integer.
///
/// # Expected Usage
///
/// The idea is on an entity to track xp you will add the `Xp` component. This component will
/// track the amount of xp accumulated. When you perform a level up action, you should call
/// `level_up` on the component. This will increase the level and reset the xp (for this level)
/// to 0. Total xp will continue to accumulate, and if there was extra xp accumulated, it will
/// be rolled over to the next level.
///
/// # Default Curve
///
/// The default curve is defined by the following values:
///
/// - `base_xp`: 10
/// - `factor_a`: 10
/// - `factor_b`: 5
/// - `factor_c`: 5
///
/// Current level will start at 1 and xp will start at 0.
///
/// If you want to define a custom curve, you should create the component with the desired values:
///
/// ```
/// use game_library::Xp;
///
/// let xp = Xp {
///   value: 0,
///   total_xp: 0,
///   current_level: 1,
///   factor_a: 50,
///   factor_b: 20,
///   factor_c: 5,
///   base_xp: 10,
/// };
/// ```
#[derive(
    Resource,
    Debug,
    Component,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Serialize,
    Hash,
    Deserialize,
    Reflect,
    InspectorOptions,
)]
#[reflect(InspectorOptions)]
pub struct Xp {
    /// Amount of xp accumulated for this level
    pub value: u32,
    /// Total amount of xp accumulated
    pub total_xp: u32,
    /// Current level of the entity
    pub current_level: u32,
    /// Scaling factor A for the xp curve
    pub factor_a: u32,
    /// Scaling factor B for the xp curve
    pub factor_b: u32,
    /// Scaling factor C for the xp curve
    pub factor_c: u32,
    /// Base xp (i.e. xp required to reach level 2)
    pub base_xp: u32,
}

impl Xp {
    /// Formula for calculating the xp required to reach a given level
    ///
    /// `(level - 1) ^ (factor_a / factor_b) / factor_c + base_xp`
    ///
    /// The result is rounded down to the nearest integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use game_library::Xp;
    ///
    /// let xp = Xp {
    ///   value: 0,
    ///   total_xp: 0,
    ///   current_level: 1,
    ///   factor_a: 10,
    ///   factor_b: 5,
    ///   factor_c: 5,
    ///   base_xp: 10,
    /// };
    ///
    /// assert_eq!(xp.xp_required(1), 10); // 11 xp to level 2
    /// assert_eq!(xp.xp_required(2), 10);
    /// assert_eq!(xp.xp_required(3), 10);
    /// assert_eq!(xp.xp_required(4), 11);
    /// assert_eq!(xp.xp_required(5), 13);
    /// assert_eq!(xp.xp_required(6), 15);
    /// ```
    #[must_use]
    pub fn xp_required(&self, level: u32) -> u32 {
        let level = if level == 0 { 0 } else { level - 1 };

        #[allow(clippy::cast_precision_loss)]
        let level = f64::from(level);
        #[allow(clippy::cast_precision_loss)]
        let factor_a = f64::from(self.factor_a);
        #[allow(clippy::cast_precision_loss)]
        let factor_b = f64::from(self.factor_b);
        #[allow(clippy::cast_precision_loss)]
        let factor_c = f64::from(self.factor_c);
        #[allow(clippy::cast_precision_loss)]
        let base_xp = f64::from(self.base_xp);

        let result = level.powf(factor_a / factor_b) / factor_c + base_xp;

        let result = result.floor();

        if result < 0. {
            return 0;
        }

        #[allow(
            clippy::cast_precision_loss,
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss
        )]
        let result = result as u32;

        result
    }

    /// Total amount of xp required to reach the next level.
    #[must_use]
    pub fn total_xp_to_next_level(&self) -> u32 {
        self.xp_required(self.current_level)
    }

    /// Remaining amount of xp required to reach the next level.
    #[must_use]
    pub fn remaining_xp_to_next_level(&self) -> u32 {
        if self.total_xp_to_next_level() == 0 {
            return 0;
        }
        if self.value > self.total_xp_to_next_level() {
            return 0;
        }

        self.total_xp_to_next_level() - self.value
    }

    /// Progress towards the next level as a percentage.
    #[must_use]
    pub fn next_level_progress(&self) -> f32 {
        #[allow(clippy::cast_precision_loss)]
        let total = self.total_xp_to_next_level() as f32;
        #[allow(clippy::cast_precision_loss)]
        let remaining = self.remaining_xp_to_next_level() as f32;
        if remaining == 0. {
            1.
        } else {
            (total - remaining) / total
        }
    }

    /// Can the entity level up?
    #[must_use]
    pub fn can_level_up(&self) -> bool {
        self.remaining_xp_to_next_level() == 0
    }

    /// Level up the entity
    pub fn level_up(&mut self) {
        if self.remaining_xp_to_next_level() > 0 {
            tracing::warn!(
                "level up rejected, not enough xp (need: {}, have: {})",
                self.total_xp_to_next_level(),
                self.value
            );
            return;
        }

        self.value -= self.total_xp_to_next_level();
        self.current_level += 1;
    }

    /// Add some amount to the xp value
    pub fn add(&mut self, amount: u32) {
        self.value += amount;
        self.total_xp += amount;
    }
}

impl AddAssign<u32> for Xp {
    #[doc = "Add some amount to the xp value"]
    fn add_assign(&mut self, rhs: u32) {
        self.add(rhs);
    }
}

impl std::default::Default for Xp {
    fn default() -> Self {
        Self {
            value: 0,
            total_xp: 0,
            current_level: 1,
            factor_a: 10,
            factor_b: 5,
            factor_c: 5,
            base_xp: 10,
        }
    }
}

impl std::fmt::Display for Xp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Level {}. Next level progress: {}/{} ({}%)",
            self.current_level,
            self.value,
            self.total_xp_to_next_level(),
            self.next_level_progress() * 100.
        )
    }
}
