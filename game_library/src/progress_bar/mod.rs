//! A 2D progress bar module for easily adding progress bars to your game.
//!
//! There are two main components in this module, a factory to create a pair of quad meshes than can be used to
//! draw a progress bar, and a component which can be added to an entity to draw the progress bar automatically.
//!

mod color_scheme;
mod config;
mod enums;
mod plugin;
mod traits;

pub use color_scheme::ColorScheme;
#[allow(clippy::module_name_repetitions)]
pub use config::ProgressBarConfig;
pub use enums::BarState;
#[allow(clippy::module_name_repetitions)]
pub use plugin::ProgressBarPlugin;
pub use traits::Percentage;
