use bevy::prelude::*;

use super::{colors, tags::SelectedOption};

/// All of the various "buttons" that can be clicked in any of the main menu screens
#[derive(Component, Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum ButtonAction {
    /// Start the game
    Play,
    /// Open the settings menu
    Settings,
    /// Quit the game
    Quit,
    /// Go back to the main menu screen
    BackToMain,
    /// Go back to the settings screen
    BackToSettings,
    // Settings sub-menu
    /// Open the display settings sub-menu
    DisplaySettings,
    /// Open the sound settings sub-menu
    SoundSettings,
    /// Open the controls settings sub-menu
    ControlsSettings,
    /// Open the gameplay settings sub-menu
    GameplaySettings,
    // Display sub-sub-menu
    /// Set the display quality
    DisplayQuality,
    // Sound sub-sub-menu
    /// Set the master volume
    VolumeLevel,
    // Controls sub-sub-menu
    /// Reset all controls to their default values
    ControlsReset,
    /// Set what button should be used to move up
    ControlsUp,
    /// Set what button should be used to move down
    ControlsDown,
    /// Set what button should be used to move left
    ControlsLeft,
    /// Set what button should be used to move right
    ControlsRight,
    /// Set what button should be used to cast the primary spell
    ControlsPrimaryCast,
    /// Set what button should be used to cast the secondary spell
    ControlsSecondaryCast,
    /// Set what button should be used to cast the defensive spell
    ControlsDefensiveCast,
    /// Set what button should be used to cast the ultimate spell
    ControlsUltimateCast,
    /// Set what button should be used to pause the game
    ControlsPause,
    /// Set what button should be used to interact with the environment / menus
    ControlsInteract,
    /// Set what button should be used to go back
    ControlsBack,
    /// Set what button should be used to toggle auto-attack
    ControlsAutoAttack,
    /// Set what button should be used to toggle auto-aim
    ControlsAutoAim,
    // Gameplay sub-sub-menu
    /// Should the game automatically cast the primary when it is ready (if auto-aim is enabled, only when a target is found)?
    GameplayAutoCastPrimary,
    /// Should the game automatically cast the secondary when it is ready (if auto-aim is enabled, only when a target is found)?
    GameplayAutoCastSecondary,
    /// Should the game automatically cast the defensive when it is ready?
    GameplayAutoCastDefensive,
    /// Should the game automatically cast the ultimate when it is ready?
    GameplayAutoCastUltimate,
    /// Change the current save game (either reset or load)
    GameplayChangeSaveGame,
}

/// This system handles changing all buttons color based on mouse interaction
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => {
                colors::PRESSED_BUTTON.into()
            }
            (Interaction::Hovered, Some(_)) => colors::HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => colors::HOVERED_BUTTON.into(),
            (Interaction::None, None) => colors::NORMAL_BUTTON.into(),
        }
    }
}

/// This system updates the settings when a new value for a setting is selected, and marks
/// the button as the one currently selected
pub fn setting_button<T: Resource + Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
    mut selected_query: Query<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
) {
    for (interaction, button_setting, entity) in &interaction_query {
        if *interaction == Interaction::Pressed && *setting != *button_setting {
            let (previous_button, mut previous_color) = selected_query.single_mut();
            *previous_color = colors::NORMAL_BUTTON.into();
            commands.entity(previous_button).remove::<SelectedOption>();
            commands.entity(entity).insert(SelectedOption);
            *setting = *button_setting;
        }
    }
}
