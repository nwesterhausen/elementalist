//! Helper functions for building the menus.
use bevy::prelude::*;

/// Make a text bundle with the given text, font, font size, and color. This is used in making
/// menu buttons.
///
/// # Parameters
///
/// * `text`: The text to display
/// * `font`: The font to use (as a handle)
/// * `font_size`: The font size to use
/// * `color`: The color to use
/// * `align_self`: The alignment of the text
///
/// # Returns
///
/// A [`TextBundle`] with the given text, font, font size, alignment and color.
#[must_use]
pub fn make_text_bundle(
    text: &str,
    font: Handle<Font>,
    font_size: f32,
    color: Color,
    align_self: AlignSelf,
) -> TextBundle {
    TextBundle {
        text: Text::from_section(
            text,
            TextStyle {
                font,
                font_size,
                color,
            },
        ),
        style: Style {
            align_self,
            ..default()
        },
        ..default()
    }
}
