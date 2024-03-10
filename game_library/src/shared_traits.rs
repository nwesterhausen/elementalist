//! Shared traits to simplify some of the object definitions.
//!

use crate::enums::CastSlot;
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

/// A trait for getting the cast slot of a spell.
pub trait KnownCastSlot {
    /// Get the cast slot of the spell.
    fn cast_slot(&self) -> CastSlot;
}

/// A trait to supply display various display information about an object.
///
/// This is useful for objects that need to be displayed in a user interface, and since
/// all object in the game are dynamically loaded, being able to use the functions in this
/// trait will allow simplifying the process of displaying the information about loaded
/// data to the user.
pub trait DisplayInformation {
    /// A tooltip that summarizes the object. Can be multiline and describes the object in brief.
    fn tooltip(&self) -> String;
    /// A longer description of the object. Can be multiline and describes the object in detail. This
    /// is used for a more detailed description of the object.
    fn more_info(&self) -> String;
    /// A one-line summary of the object. This puts it in a single line for lists and other places
    /// where a single line is needed.
    fn summarized_one_liner(&self) -> String;
}
