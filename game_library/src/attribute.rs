use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Attribute component. This is a simple integer representing the attribute of an entity.
/// Has properties like `max_attribute` and `current_attribute`. We support operations with
/// integers, floats, and other `Attribute` components.
///
/// We use positive integers for attribute, and negative integers for damage. There's no reason
/// to support negative attribute and negative max_attribute, so we don't. We also won't allow
/// `current_attribute` to be greater than `max_attribute`.
///
/// Because we use integers, we can't represent fractional attribute. It's a bit of a tradeoff,
/// but it's a design decision (and I guess it can be changed later at great effort).
///
/// All operations that are performed directly between a `Attribute` component and another value will
/// use the `current_attribute` value. For example, if you add a `Attribute` component with `current_attribute`
/// of 10 to a `Attribute` component with `current_attribute` of 20, the result will be a `Attribute` component
/// with `current_attribute` of 30. This holds true for all operations (addition, subtraction, multiplication,
/// division, etc.).
///
/// What is supported:
///
/// * Affecting (adding/subtracting) attribute with integers, floats, and other `Attribute` components.
/// * Scaling the current attribute with integers, floats, and other `Attribute` components
/// * Scaling the max attribute with integers, floats, and other `Attribute` components
/// * Scaling the current attribute by a percentage of the max attribute (with integers, floats, and other `Attribute` components)
/// * Comparing attribute with integers, floats, and other `Attribute` components.
/// * Getting the percentage of attribute remaining (as a rounded integer between 0 and 100 OR a precise
/// float between 0 and 1).
/// * Check `is_empty` to see if the entity is dead (i.e. `current_attribute` is 0).
/// * Check `is_full` to see if the entity is at full attribute (i.e. `current_attribute` is equal to `max_attribute`).
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Attribute {
    /// The maximum attribute of the entity
    pub max_attribute: u32,
    /// The current attribute of the entity
    pub current_attribute: u32,
}

impl Attribute {
    /// Minimum value is 0
    pub const MIN: u32 = 0;
    /// Maximum value is 2^32 - 1 (u32::MAX)
    pub const MAX: u32 = std::u32::MAX;

    /// Returns true if the current attribute is equal to 0
    pub fn is_empty(&self) -> bool {
        self.current_attribute == 0
    }

    /// Returns true if the current attribute is equal to the max attribute.
    ///
    /// ## Note
    ///
    /// If max attribute is 0, this will always return true (because current attribute will always be 0
    /// if max attribute is 0)
    pub fn is_full(&self) -> bool {
        self.current_attribute == self.max_attribute
    }

    /// Returns the percentage of attribute remaining as a float between 0.00 and 1.00
    ///
    /// ## Note
    ///
    /// This will always return 1.00 if max attribute is 0 (because current attribute will always be 0
    /// if max attribute is 0)
    pub fn remaining(&self) -> f64 {
        // Avoid division by zero
        if self.max_attribute == 0 {
            return 1.0;
        }
        // Get percentage but round to 2 decimal places
        (self.current_attribute as f64 / self.max_attribute as f64 * 100.0).round() / 100.0
    }

    /// Returns the percentage of attribute remaining as an integer between 0% and 100%
    ///
    /// ## Note
    ///
    /// This will always return 100% if max attribute is 0 (because current attribute will always be 0
    /// if max attribute is 0)
    pub fn percentage_remaining(&self) -> u32 {
        // Avoid division by zero
        if self.max_attribute == 0 {
            return 100;
        }
        // Percentage but floored and as an integer between 0 and 100
        (self.current_attribute as f64 / self.max_attribute as f64 * 100.0).floor() as u32
    }

    /// Creates a new `Attribute` component with the given `max_attribute` and `current_attribute`.
    ///
    /// ## Arguments
    ///
    /// * `max_attribute` - The maximum attribute of the entity. This will be converted into a `u32` and clamped
    pub fn new(max_attribute: impl Into<u32>) -> Self {
        let clamped_max_attribute = max_attribute.into().min(Self::MAX).max(Self::MIN);
        Self {
            max_attribute: clamped_max_attribute,
            current_attribute: clamped_max_attribute,
        }
    }

    /// Creates a new `Attribute` component with the given `max_attribute` and `current_attribute`.
    ///
    /// ## Arguments
    ///
    /// * `current_attribute` - The current attribute of the entity. This will be converted into a `u32` and clamped
    /// * `max_attribute` - The maximum attribute of the entity. This will be converted into a `u32` and clamped
    pub fn new_with_current_attribute(
        current_attribute: impl Into<u32>,
        max_attribute: impl Into<u32>,
    ) -> Self {
        let clamped_current_attribute = current_attribute.into().min(Self::MAX).max(Self::MIN);
        let clamped_max_attribute = max_attribute.into().min(Self::MAX).max(Self::MIN);
        Self {
            max_attribute: clamped_max_attribute,
            current_attribute: clamped_current_attribute,
        }
    }

    /// Adds the given amount to the max attribute, while clamping max attribute within acceptable bounds.
    /// Also ensures that `current_attribute` is not greater than `max_attribute` (in cases where a negative
    /// amount is added here).
    ///
    /// ## Arguments
    ///
    /// * `amount` - The amount to add to the max attribute. This can be a positive or negative number, which
    /// will increase or decrease the max attribute respectively.
    pub fn add_to_max_attribute(&mut self, amount: impl Into<i64>) {
        let amount = amount.into();
        let new_max_attribute = self.max_attribute as i64 + amount;
        self.max_attribute = new_max_attribute
            .min(Self::MIN as i64)
            .max(Self::MAX as i64) as u32;
        self.current_attribute = self.current_attribute.max(self.max_attribute);
    }

    /// Scales the max attribute by the given amount, while clamping max attribute within acceptable bounds. Also
    /// ensures that `current_attribute` is not greater than `max_attribute` (in cases where the scale value is
    /// less than 1; i.e. the max attribute is decreased).
    ///
    /// ## Arguments
    ///
    /// * `amount` - The amount to scale the max attribute by. This should be a positive number. See the sister
    /// function `scale_max_attribute_by_percentage` for a function that takes a percentage (integer value).
    ///
    /// ## Note
    ///
    /// - Really large numbers will just cause max attribute to be set to `Attribute::MAX`.
    /// - Negative numbers will cause max attribute to be set to 0.
    /// - This is a sister function to `scale_max_attribute_by_percentage` which takes a percentage value instead
    pub fn scale_max_attribute(&mut self, amount: impl Into<f64> + std::cmp::PartialOrd<f64>) {
        // Guard against negative numbers (which would cause the max attribute to be negative)
        if amount < 0.0 {
            tracing::warn!("refused to scale by a negative number; set max_attribute to 0 instead");
            self.max_attribute = 0;
            self.current_attribute = 0;
            return;
        }

        let amount = amount.into();
        let new_max_attribute = self.max_attribute as f64 * amount;

        self.max_attribute = new_max_attribute
            .min(Self::MIN as f64)
            .max(Self::MAX as f64) as u32;
        self.current_attribute = self.current_attribute.max(self.max_attribute);
    }

    /// Scales the max attribute by the given percentage, while clamping max attribute within acceptable bounds. Also
    /// ensures that `current_attribute` is not greater than `max_attribute` (in cases where the scale value is
    /// less than 100; i.e. the max attribute is decreased).
    ///
    /// ## Arguments
    ///
    /// * `amount` - The percentage to scale the max attribute by. This should be a positive number. Really large numbers
    /// will just cause max attribute to be set to `u32::MAX`.
    ///
    /// ## Note
    ///
    /// - Really large numbers will just cause max attribute to be set to `Attribute::MAX`.
    /// - Negative numbers will cause max attribute to be set to 0.
    /// - This is a sister function to `scale_max_attribute` which takes a float value instead of a percentage.
    pub fn scale_max_attribute_by_percentage(&mut self, amount: impl Into<i32>) {
        self.scale_max_attribute(amount.into() as f64 / 100.0);
    }

    /// Adds the given amount to the current attribute, while clamping current attribute within acceptable bounds.
    /// (i.e. between 0 and `max_attribute`). This will not affect `max_attribute`.
    ///
    /// ## Arguments
    ///
    /// * `amount` - The amount to add to the current attribute. This can be a positive or negative number, which
    /// will increase or decrease the current attribute respectively.
    pub fn add_to_current_attribute(&mut self, amount: impl Into<i64>) {
        let amount = amount.into();
        let new_current_attribute = self.current_attribute as i64 + amount;
        self.current_attribute = new_current_attribute
            .min(Self::MIN as i64)
            .max(self.max_attribute as i64) as u32;
    }

    /// Scales the current attribute by the given amount, while clamping current attribute within acceptable bounds.
    /// (i.e. between 0 and `max_attribute`). This will not affect `max_attribute`. This takes a float value; e.g.,
    /// `0.5` will scale the current attribute by 50% (i.e. half the current attribute), while `2.0` will scale the
    /// current attribute by 200% (i.e. double the current attribute).
    ///
    /// There's a similar function `add_to_current_attribute_by_scale_max_attribute` which applies the same logic but
    /// uses the `max_attribute` as the scale value to arrive at an amount to change the current attribute by.
    ///
    /// ## Arguments
    ///
    /// * `amount` - The amount to scale the current attribute by. This should be a positive number. See the sister
    /// function `scale_current_attribute_by_percentage` for a function that takes a percentage (integer value).
    pub fn scale_current_attribute(&mut self, amount: impl Into<f64> + std::cmp::PartialOrd<f64>) {
        // Guard against negative numbers (which would cause the current attribute to be negative)
        if amount < 0.0 {
            tracing::warn!(
                "refused to scale by a negative number; set current_attribute to 0 instead"
            );
            self.current_attribute = 0;
            return;
        }

        let amount = amount.into();
        let new_current_attribute = self.current_attribute as f64 * amount;

        self.current_attribute = new_current_attribute
            .min(Self::MIN as f64)
            .max(self.max_attribute as f64) as u32;
    }

    /// Scales the current attribute by the given percentage, while clamping current attribute within acceptable bounds.
    ///
    /// There's a similar function `add_to_current_attribute_by_scale_max_attribute_percentage` which applies the same logic but
    /// uses the `max_attribute` as the scale value to arrive at an amount to change the current attribute by.
    ///
    /// ## Arguments
    ///
    /// * `amount` - The percentage to scale the current attribute by. This should be a positive number. Really large numbers
    /// will just cause current attribute to be set to `u32::MAX`.
    pub fn scale_current_attribute_by_percentage(&mut self, amount: impl Into<i32>) {
        self.scale_current_attribute(amount.into() as f64 / 100.0);
    }

    /// Adds to current attribute by scaling the `max_attribute` by the given amount, while clamping current attribute within
    /// acceptable bounds. (i.e. between 0 and `max_attribute`). This will not affect `max_attribute`. This takes a float
    /// and expects negatives to be used to decrease the current attribute, and positives to be used to increase the
    /// current attribute.
    ///
    /// There's a similar function `scale_current_attribute_by_scale_max_attribute_percentage` which applies the same logic but
    /// expects a percentage value (e.g. 50) instead of a float (e.g. 0.5).
    ///
    /// ## Arguments
    ///
    /// * `amount` - The amount to scale the max attribute by. If this is positive, the current attribute will increase. If
    /// this is negative, the current attribute will decrease.
    pub fn add_to_current_attribute_by_scale_max_attribute(
        &mut self,
        amount: impl Into<f64> + std::cmp::PartialOrd<f64>,
    ) {
        // Turn the amount into an i64 so we can easily affect the current attribute
        let amount = amount.into() * self.max_attribute as f64;
        self.add_to_current_attribute(amount as i64);
    }

    /// Adds to current attribute by scaling the `max_attribute` by the given percentage, while clamping current attribute within
    /// acceptable bounds. (i.e. between 0 and `max_attribute`). This will not affect `max_attribute`. This takes a percentage
    /// and expects negatives to be used to decrease the current attribute, and positives to be used to increase the
    /// current attribute.
    ///
    /// There's a similar function `add_to_current_attribute_by_scale_max_attribute` which applies the same logic but
    /// expects a float value (e.g. 0.5) instead of a percentage (e.g. 50).
    ///
    /// ## Arguments
    ///
    /// * `amount` - The percentage to scale the max attribute by. If this is positive, the current attribute will increase. If
    /// this is negative, the current attribute will decrease.
    pub fn add_to_current_attribute_by_scale_max_attribute_percentage(
        &mut self,
        amount: impl Into<i32>,
    ) {
        self.add_to_current_attribute_by_scale_max_attribute(amount.into() as f64 / 100.0);
    }
}

impl std::ops::Add<u32> for Attribute {
    type Output = Self;

    fn add(mut self, rhs: u32) -> Self::Output {
        self.add_to_current_attribute(rhs);
        self
    }
}

impl std::ops::AddAssign<u32> for Attribute {
    fn add_assign(&mut self, rhs: u32) {
        self.add_to_current_attribute(rhs);
    }
}

impl std::ops::Add<i32> for Attribute {
    type Output = Self;

    fn add(mut self, rhs: i32) -> Self::Output {
        self.add_to_current_attribute(rhs);
        self
    }
}

impl std::ops::AddAssign<i32> for Attribute {
    fn add_assign(&mut self, rhs: i32) {
        self.add_to_current_attribute(rhs);
    }
}

impl std::ops::Add<f32> for Attribute {
    type Output = Self;

    fn add(mut self, rhs: f32) -> Self::Output {
        self.add_to_current_attribute(rhs as i32);
        self
    }
}

impl std::ops::AddAssign<f32> for Attribute {
    fn add_assign(&mut self, rhs: f32) {
        self.add_to_current_attribute(rhs as i32);
    }
}

impl std::ops::Add<f64> for Attribute {
    type Output = Self;

    fn add(mut self, rhs: f64) -> Self::Output {
        self.add_to_current_attribute(rhs as i32);
        self
    }
}

impl std::ops::AddAssign<f64> for Attribute {
    fn add_assign(&mut self, rhs: f64) {
        self.add_to_current_attribute(rhs as i32);
    }
}

impl std::ops::Add<Attribute> for Attribute {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.add_to_current_attribute(rhs.current_attribute);
        self
    }
}

impl std::ops::AddAssign<Attribute> for Attribute {
    fn add_assign(&mut self, rhs: Self) {
        self.add_to_current_attribute(rhs.current_attribute);
    }
}

impl std::ops::Sub<u32> for Attribute {
    type Output = Self;

    fn sub(mut self, rhs: u32) -> Self::Output {
        self.add_to_current_attribute(-(rhs as i64));
        self
    }
}

impl std::ops::SubAssign<u32> for Attribute {
    fn sub_assign(&mut self, rhs: u32) {
        self.add_to_current_attribute(-(rhs as i64));
    }
}

impl std::ops::Sub<i32> for Attribute {
    type Output = Self;

    fn sub(mut self, rhs: i32) -> Self::Output {
        self.add_to_current_attribute(-rhs as i64);
        self
    }
}

impl std::ops::SubAssign<i32> for Attribute {
    fn sub_assign(&mut self, rhs: i32) {
        self.add_to_current_attribute(-rhs as i64);
    }
}

impl std::ops::Sub<f32> for Attribute {
    type Output = Self;

    fn sub(mut self, rhs: f32) -> Self::Output {
        self.add_to_current_attribute(-rhs as i64);
        self
    }
}

impl std::ops::SubAssign<f32> for Attribute {
    fn sub_assign(&mut self, rhs: f32) {
        self.add_to_current_attribute(-rhs as i64);
    }
}

impl std::ops::Sub<f64> for Attribute {
    type Output = Self;

    fn sub(mut self, rhs: f64) -> Self::Output {
        self.add_to_current_attribute(-rhs as i64);
        self
    }
}

impl std::ops::SubAssign<f64> for Attribute {
    fn sub_assign(&mut self, rhs: f64) {
        self.add_to_current_attribute(-rhs as i64);
    }
}

impl std::ops::Sub<Attribute> for Attribute {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.add_to_current_attribute(-(rhs.current_attribute as i64));
        self
    }
}

impl std::ops::SubAssign<Attribute> for Attribute {
    fn sub_assign(&mut self, rhs: Self) {
        self.add_to_current_attribute(-(rhs.current_attribute as i64));
    }
}

impl std::ops::Mul<u32> for Attribute {
    type Output = Self;

    fn mul(mut self, rhs: u32) -> Self::Output {
        self.scale_current_attribute(rhs as f64);
        self
    }
}

impl std::ops::MulAssign<u32> for Attribute {
    fn mul_assign(&mut self, rhs: u32) {
        self.scale_current_attribute(rhs as f64);
    }
}

impl std::ops::Mul<i32> for Attribute {
    type Output = Self;

    fn mul(mut self, rhs: i32) -> Self::Output {
        self.scale_current_attribute(rhs as f64);
        self
    }
}

impl std::ops::MulAssign<i32> for Attribute {
    fn mul_assign(&mut self, rhs: i32) {
        self.scale_current_attribute(rhs as f64);
    }
}

impl std::ops::Mul<f32> for Attribute {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self.scale_current_attribute(rhs as f64);
        self
    }
}

impl std::ops::MulAssign<f32> for Attribute {
    fn mul_assign(&mut self, rhs: f32) {
        self.scale_current_attribute(rhs as f64);
    }
}

impl std::ops::Mul<f64> for Attribute {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self.scale_current_attribute(rhs);
        self
    }
}

impl std::ops::MulAssign<f64> for Attribute {
    fn mul_assign(&mut self, rhs: f64) {
        self.scale_current_attribute(rhs);
    }
}

impl std::ops::Mul<Attribute> for Attribute {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self.scale_current_attribute(rhs.current_attribute as f64);
        self
    }
}

impl std::ops::MulAssign<Attribute> for Attribute {
    fn mul_assign(&mut self, rhs: Self) {
        self.scale_current_attribute(rhs.current_attribute as f64);
    }
}

impl std::ops::Div<u32> for Attribute {
    type Output = Self;

    fn div(mut self, rhs: u32) -> Self::Output {
        if rhs == 0 {
            tracing::error!("avoided division by zero; set current_attribute to 0 instead");
            self.current_attribute = 0;
            return self;
        }
        self.scale_current_attribute(1.0 / rhs as f64);
        self
    }
}

impl std::ops::DivAssign<u32> for Attribute {
    fn div_assign(&mut self, rhs: u32) {
        if rhs == 0 {
            tracing::error!("avoided division by zero; set current_attribute to 0 instead");
            self.current_attribute = 0;
        }
        self.scale_current_attribute(1.0 / rhs as f64);
    }
}

impl std::ops::Div<i32> for Attribute {
    type Output = Self;

    fn div(mut self, rhs: i32) -> Self::Output {
        if rhs <= 0 {
            tracing::error!("avoided division by zero; set current_attribute to 0 instead");
            self.current_attribute = 0;
            return self;
        }
        self.scale_current_attribute(1.0 / rhs as f64);
        self
    }
}

impl std::ops::DivAssign<i32> for Attribute {
    fn div_assign(&mut self, rhs: i32) {
        if rhs <= 0 {
            tracing::error!("avoided division by zero; set current_attribute to 0 instead");
            self.current_attribute = 0;
        }
        self.scale_current_attribute(1.0 / rhs as f64);
    }
}

impl std::ops::Div<f32> for Attribute {
    type Output = Self;

    fn div(mut self, rhs: f32) -> Self::Output {
        if rhs <= 0.0 {
            tracing::error!("avoided division by zero; set current_attribute to 0 instead");
            self.current_attribute = 0;
            return self;
        }
        self.scale_current_attribute(1.0 / rhs as f64);
        self
    }
}

impl std::ops::DivAssign<f32> for Attribute {
    fn div_assign(&mut self, rhs: f32) {
        if rhs <= 0.0 {
            tracing::error!("avoided division by zero; set current_attribute to 0 instead");
            self.current_attribute = 0;
        }
        self.scale_current_attribute(1.0 / rhs as f64);
    }
}

impl std::ops::Div<f64> for Attribute {
    type Output = Self;

    fn div(mut self, rhs: f64) -> Self::Output {
        if rhs <= 0.0 {
            tracing::error!("avoided division by zero; set current_attribute to 0 instead");
            self.current_attribute = 0;
            return self;
        }
        self.scale_current_attribute(1.0 / rhs);
        self
    }
}

impl std::ops::DivAssign<f64> for Attribute {
    fn div_assign(&mut self, rhs: f64) {
        if rhs <= 0.0 {
            tracing::error!("avoided division by zero; set current_attribute to 0 instead");
            self.current_attribute = 0;
        }
        self.scale_current_attribute(1.0 / rhs);
    }
}

impl std::ops::Div<Attribute> for Attribute {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        if rhs.current_attribute == 0 {
            tracing::error!("avoided division by zero; set current_attribute to 0 instead");
            self.current_attribute = 0;
            return self;
        }
        self.scale_current_attribute(1.0 / rhs.current_attribute as f64);
        self
    }
}

impl std::ops::DivAssign<Attribute> for Attribute {
    fn div_assign(&mut self, rhs: Self) {
        if rhs.current_attribute == 0 {
            tracing::error!("avoided division by zero; set current_attribute to 0 instead");
            self.current_attribute = 0;
        }
        self.scale_current_attribute(1.0 / rhs.current_attribute as f64);
    }
}

impl std::cmp::PartialOrd<Attribute> for Attribute {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.current_attribute.partial_cmp(&other.current_attribute)
    }
}

impl std::cmp::Ord for Attribute {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.current_attribute.cmp(&other.current_attribute)
    }
}

impl From<u32> for Attribute {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<i32> for Attribute {
    fn from(value: i32) -> Self {
        if value < 0 {
            tracing::warn!("refused to create Attribute with negative value; set to 0 instead");
            return Self::new(0_u32);
        }
        Self::new(value as u32)
    }
}

impl From<f32> for Attribute {
    fn from(value: f32) -> Self {
        // Floor the value to get an integer
        let value = value.floor() as i32;
        if value < 0 {
            tracing::warn!("refused to create Attribute with negative value; set to 0 instead");
            return Self::new(0_u32);
        }
        Self::new(value as u32)
    }
}

impl From<f64> for Attribute {
    fn from(value: f64) -> Self {
        // Floor the value to get an integer
        let value = value.floor() as i32;
        if value < 0 {
            tracing::warn!("refused to create Attribute with negative value; set to 0 instead");
            return Self::new(0_u32);
        }
        Self::new(value as u32)
    }
}

impl From<Attribute> for u32 {
    fn from(value: Attribute) -> Self {
        value.current_attribute
    }
}

impl From<Attribute> for i32 {
    fn from(value: Attribute) -> Self {
        value.current_attribute as i32
    }
}

impl From<Attribute> for f32 {
    fn from(value: Attribute) -> Self {
        value.current_attribute as f32
    }
}

impl From<Attribute> for f64 {
    fn from(value: Attribute) -> Self {
        value.current_attribute as f64
    }
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{}",
            self.current_attribute.to_string(),
            self.max_attribute.to_string()
        )
    }
}
