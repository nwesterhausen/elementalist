//! A resource to hold handles for the fonts used in the game.
//!
//! This is intended to make it trivial to change which font face is used for
//! each type of text in the game. For example, if the user wanted to make it
//! easier to read the text in the game, they could change the font face for
//! the text in the game to a more readable font face.

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use serde::{Deserialize, Serialize};

/// A resource to hold handles for the fonts used in the game.
#[derive(Resource, Debug, Clone, Default, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct FontResource {
    /// The font face used for "display" text.
    ///
    /// This should probably never be changed, as it is used for the game logo.
    pub display_font: Handle<Font>,
    /// The font face used for menus and other interface elements.
    pub interface_font: Handle<Font>,
    /// The font face used for the main text in the game.
    pub main_font: Handle<Font>,
    /// The font face used for the console.
    ///
    /// This should probably never be changed, as it is used for the console.
    pub console_font: Handle<Font>,
    /// The font handle for FontFamily::Display
    pub display_font_handle: Handle<Font>,
    /// The font handle for FontFamily::Fancy
    pub fancy_font_handle: Handle<Font>,
    /// The font handle for FontFamily::Dyslexic
    pub dyslexic_font_handle: Handle<Font>,
    /// The font handle for FontFamily::SansSerif
    pub sans_serif_font_handle: Handle<Font>,
    /// The font handle for FontFamily::Monospace
    pub monospace_font_handle: Handle<Font>,
}

impl FontResource {
    /// Get the font handle for the specified font family.
    #[must_use]
    pub fn get_font_handle(&self, font_family: FontFamily) -> Handle<Font> {
        match font_family {
            FontFamily::Display => self.display_font_handle.clone(),
            FontFamily::Fancy => self.fancy_font_handle.clone(),
            FontFamily::Dyslexic => self.dyslexic_font_handle.clone(),
            FontFamily::SansSerif => self.sans_serif_font_handle.clone(),
            FontFamily::Monospace => self.monospace_font_handle.clone(),
        }
    }
}

/// A choice of font to change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub enum FontChoice {
    /// The font face used for "display" text.
    Display,
    /// The font face used for menus and other interface elements.
    Interface,
    /// The font face used for the main text in the game.
    Main,
    /// The font face used for the console.
    Console,
    /// All of the font faces.
    All,
}

/// The generic font-family options. This is used in [`crate::settings::AccessibilitySettings`]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Reflect,
    InspectorOptions,
    Serialize,
    Deserialize,
    Default,
)]
#[reflect(InspectorOptions)]
pub enum FontFamily {
    /// The display font (for the game logo)
    Display,
    /// "Fancy" display text.
    #[default]
    Fancy,
    /// OpenDyslexic
    Dyslexic,
    /// Sans-serif
    SansSerif,
    /// Monospace
    Monospace,
}

impl std::fmt::Display for FontFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Display | Self::Fancy => write!(f, "Fancy"),
            Self::Dyslexic => write!(f, "OpenDyslexic"),
            Self::SansSerif => write!(f, "Sans-Serif"),
            Self::Monospace => write!(f, "Monospace"),
        }
    }
}

/// Change a font choice. Sending this event will update the specified font
/// choice to the specified font.
#[derive(Event)]
pub struct ChangeFont {
    /// The font choice to change.
    pub font_choice: FontChoice,
    /// The new font to use for the specified font choice.
    pub new_font: Handle<Font>,
}

/// A system to change the font choice.
///
/// This system will change the font choice to the specified font, when the
/// [`ChangeFont`] event is sent.
///
/// Ensure that the [`FontResource`] is inserted into the app resources before
/// this system is run, otherwise bevy will panic.
#[allow(dead_code)]
pub fn change_font(
    mut font_resource: ResMut<FontResource>,
    mut change_font_events: EventReader<ChangeFont>,
) {
    for change_font_event in change_font_events.read() {
        match change_font_event.font_choice {
            FontChoice::Display => {
                font_resource.display_font = change_font_event.new_font.clone();
            }
            FontChoice::Interface => {
                font_resource.interface_font = change_font_event.new_font.clone();
            }
            FontChoice::Main => {
                font_resource.main_font = change_font_event.new_font.clone();
            }
            FontChoice::Console => {
                font_resource.console_font = change_font_event.new_font.clone();
            }
            FontChoice::All => {
                font_resource.interface_font = change_font_event.new_font.clone();
                font_resource.main_font = change_font_event.new_font.clone();
            }
        }
    }
}
