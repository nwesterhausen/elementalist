//! Resources are stored here for easy access via `crate::resources::ResourceName`.
//! Systems for the resources may be here or in the specific system that needs them.
//!
//! All resources can be added with default values to the game using the `ElementalistResourcesPlugin`.

mod cleanup;
mod cursor_position;
mod fonts;
mod plugin;

pub mod movement;

pub use plugin::ElementalistResourcesPlugin;
