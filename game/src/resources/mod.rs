//! Resources are stored here for easy access via `crate::resources::ResourceName`.
//! Systems for the resources may be here or in the specific system that needs them.
//!
//! All resources can be added with default values to the game using the `ElementalistResourcesPlugin`.

mod app_state;
mod cleanup;
mod cursor_position;
mod fonts;
mod particles;
mod plugin;
mod return_to_state;
mod state;

pub mod buttons;
pub mod colors;
pub mod movement;

pub use app_state::AppState;
pub use plugin::ElementalistResourcesPlugin;
pub use return_to_state::ReturnToState;
pub use state::*;
