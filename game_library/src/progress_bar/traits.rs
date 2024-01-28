//! Traits used in the progress bar module.

/// Trait for accessing the percentage of a value.
pub trait Percentage {
    /// Returns the percentage of the value to draw.
    fn percentage(&self) -> f32;
}
