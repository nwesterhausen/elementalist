//! A sub-set of colors from the color pallette used in the game.

use bevy::prelude::*;

/// The dark color used for the background.
///
/// <div style="background-color:rgb(17%, 13%, 17%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #2b222a
/// pub const BACKGROUND_COLOR: Color = Color::rgba(0.169, 0.133, 0.165, 1.);
/// The dark color used for the background at 50% opacity.
pub const BACKGROUND_COLOR_50: Color = Color::rgba(0.169, 0.133, 0.165, 0.5);

/// The clear color used for the game.
///
/// <div style="background-color:rgb(10%, 12%, 18%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #1a1f2e
pub const CLEAR_COLOR: Color = Color::rgba(0.102, 0.122, 0.18, 1.);

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

/// The color used for the health bar.
///
/// <div style="background-color:rgb(77%, 23%, 25%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #c33c40
pub const HEALTH_BAR_COLOR: Color = Color::rgba(0.765, 0.235, 0.251, 1.);

/// Health bar Ok
///
/// <div style="background-color:rgb(37%, 50%, 20%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #5f7132
pub const HEALTH_BAR_COLOR_OK: Color = Color::rgba(0.373, 0.502, 0.196, 1.);

/// Health bar Moderate
///
/// <div style="background-color:rgb(55%, 28%, 19%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #8d4830
pub const HEALTH_BAR_COLOR_MODERATE: Color = Color::rgba(0.553, 0.282, 0.188, 1.);

/// Health bar Critical
///
/// <div style="background-color:rgb(77%, 23%, 25%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #c33c40
pub const HEALTH_BAR_COLOR_CRITICAL: Color = HEALTH_BAR_COLOR;

/// The color used for the mana bar.
///
/// <div style="background-color:rgb(22%, 16%, 37%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #39275e
pub const MANA_BAR_COLOR: Color = Color::rgba(0.224, 0.153, 0.369, 1.);

/// The color used for the skill track bar.
///
/// <div style="background-color:rgb(77%, 47%, 21%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #c57835
pub const SKILL_TRACK_BAR_COLOR: Color = Color::rgba(0.773, 0.471, 0.208, 1.);
