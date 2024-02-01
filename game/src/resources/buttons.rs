use bevy::prelude::*;

use super::colors;

/// Tag component used to mark which setting is currently selected
#[derive(Component)]
pub struct SelectedOption;

/// System for changing button colors when hovered, etc
///
/// * `interaction_query`: grabs all the buttons that have been interacted with, with the components
///   Interaction, children, and if they are a selected option (e.g. part of a radio group). It grabs
///   what has changed about the interaction (i.e. if it has changed at all)
/// * `text_query`: let's us grab the text component of the button
#[allow(clippy::type_complexity)]
pub fn interaction_system(
    mut interaction_query: Query<
        (&Interaction, &Children, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    // Loop through all the buttons that have been interacted with (and all the components we grabbed)
    for (interaction, children, selected) in &mut interaction_query {
        // Try to grab the text component of the button, and if we can't we will just skip (continue)
        // We are changing the text color and not the button color because drawing big squares is dumb. :)
        let Ok(mut text) = text_query.get_mut(children[0]) else {
            tracing::error!("Button has no text (and was probably supposed to)");
            continue;
        };
        // Match the interaction and selected option to change the text color
        // i.e., hover, not-hover & selected, pressed, and not-pressed & not selected. (or close that that)
        match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => {
                text.sections[0].style.color = colors::SELECTED_TEXT_COLOR;
            }
            (Interaction::Hovered, Some(_)) => {
                text.sections[0].style.color = colors::HOVERED_TEXT_COLOR_ALTERNATE;
            }
            (Interaction::Hovered, None) => {
                text.sections[0].style.color = colors::HOVERED_TEXT_COLOR;
            }
            (Interaction::None, None) => text.sections[0].style.color = colors::TEXT_COLOR,
        }
    }
}
