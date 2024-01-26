//! A sub-set of colors from the color pallette used in the game.

use bevy::prelude::*;

/// The dark color used for the background.
///
/// <div style="background-color:rgb(17%, 13%, 17%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #2b222a
pub const BACKGROUND_COLOR: Color = Color::rgba(0.169, 0.133, 0.165, 1.);
/// The dark color used for the background at 50% opacity.
pub const BACKGROUND_COLOR_50: Color = Color::rgba(0.169, 0.133, 0.165, 0.5);

/// The clear color used for the game.
///
/// <div style="background-color:rgb(10%, 12%, 18%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #1a1f2e
pub const CLEAR_COLOR: Color = Color::rgba(0.102, 0.122, 0.18, 1.);
/// The clear color used for the game at 50% opacity.
pub const CLEAR_COLOR_50: Color = Color::rgba(0.102, 0.122, 0.18, 0.5);

/// The color used for text.
///
/// <div style="background-color:rgb(100%, 66%, 27%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #e1a845
pub const TEXT_COLOR: Color = Color::rgba(1., 0.658, 0.27, 1.);

/// The color used for selected text.
///
/// <div style="background-color:rgb(90%, 94%, 94%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #e5efef
pub const SELECTED_TEXT_COLOR: Color = Color::rgba(0.898, 0.937, 0.937, 1.);

/// The color used for hovered text.
///
/// <div style="background-color:rgb(93%, 95%, 48%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #eff37c
pub const HOVERED_TEXT_COLOR: Color = Color::rgba(0.937, 0.953, 0.486, 1.);

/// The color used for hovered text (alternate).
///
/// <div style="background-color:rgb(55%, 28%, 19%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #8d4830
pub const HOVERED_TEXT_COLOR_ALTERNATE: Color = Color::rgba(0.553, 0.282, 0.188, 1.);
