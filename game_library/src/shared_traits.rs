//! Shared traits to simplify some of the object definitions.
//!
/// Trait for objects that have an internal ID.
///
/// This is useful for objects that need to be identified by a unique ID.
pub trait InternalId: Sized {
    /// Get the object's internal ID.
    #[must_use]
    fn get_internal_id(&self) -> String;
    /// Update the object's internal ID.
    fn update_internal_id(&mut self);
}
