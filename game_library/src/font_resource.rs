//! A resource to hold handles for the fonts used in the game.
//!
//! This is intended to make it trivial to change which font face is used for
//! each type of text in the game. For example, if the user wanted to make it
//! easier to read the text in the game, they could change the font face for
//! the text in the game to a more readable font face.

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

/// A resource to hold handles for the fonts used in the game.
#[derive(Resource, Debug, Clone, Default, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct FontResource {
    /// The font face used for "display" text.
    pub display_font: Handle<Font>,
    /// The font face used for menus and other interface elements.
    pub interface_font: Handle<Font>,
    /// The font face used for the main text in the game.
    pub main_font: Handle<Font>,
    /// The font face used for the console.
    pub console_font: Handle<Font>,
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
        }
    }
}
