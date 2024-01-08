//! Resources are stored here for easy access via `crate::resources::ResourceName`.
//! Systems for the resources may be here or in the specific system that needs them.
//!
//! All resources can be added with default values to the game using the `ElementalistResourcesPlugin`.

mod cursor_position;
mod plugin;

pub use cursor_position::*;
pub use plugin::ElementalistResourcesPlugin;
