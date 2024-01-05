use bevy::prelude::*;

use super::{colors, tags::SelectedOption};

/// All of the various "buttons" that can be clicked in any of the main menu screens
#[derive(Component, Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum ButtonAction {
    /// Start the game
    MainMenuPlay,
    /// Open the settings menu
    MainMenuSettings,
    /// Quit the game
    MainMenuQuit,
    /// Go back to the main menu screen
    BackToMainMenu,
    /// Go back to the settings screen
    BackToSettings,
    // Settings sub-menu
    /// Open the display settings sub-menu
    SettingsDisplay,
    /// Open the sound settings sub-menu
    SettingsSound,
    /// Open the controls settings sub-menu
    SettingsControls,
    /// Open the gameplay settings sub-menu
    SettingsGameplay,
    // Display sub-sub-menu
    /// Set the display quality
    SetDisplayQuality,
    // Sound sub-sub-menu
    /// Set the game volume
    SetGameVolume(u32),
    /// Set the music volume
    SetMusicVolume(u32),
    /// Set the sound effects volume
    SetSoundEffectsVolume(u32),
    /// Toggle whether or not game audio is muted
    SetGameMuted,
    /// Toggle whether or not music is muted
    SetMusicMuted,
    /// Toggle whether or not sound effects are muted
    SetSoundEffectsMuted,
    // Controls sub-sub-menu
    /// Reset all controls to their default values
    ResetControlsToDefault,
    /// Set what button should be used to move up
    SetControlsUp,
    /// Set what button should be used to move down
    SetControlsDown,
    /// Set what button should be used to move left
    SetControlsLeft,
    /// Set what button should be used to move right
    SetControlsRight,
    /// Set what button should be used to cast the primary spell
    SetControlsPrimaryCast,
    /// Set what button should be used to cast the secondary spell
    SetControlsSecondaryCast,
    /// Set what button should be used to cast the defensive spell
    SetControlsDefensiveCast,
    /// Set what button should be used to cast the ultimate spell
    SetControlsUltimateCast,
    /// Set what button should be used to pause the game
    SetControlsPause,
    /// Set what button should be used to interact with the environment / menus
    SetControlsAcceptInteract,
    /// Set what button should be used to go back
    SetControlsBackCancel,
    /// Set what button should be used to toggle auto-attack
    SetControlsAutoAttack,
    /// Set what button should be used to toggle auto-aim
    SetControlsAutoAim,
    // Gameplay sub-sub-menu
    /// Should the game automatically cast the primary when it is ready (if auto-aim is enabled, only when a target is found)?
    SetGameplayAutoCastPrimary,
    /// Should the game automatically cast the secondary when it is ready (if auto-aim is enabled, only when a target is found)?
    SetGameplayAutoCastSecondary,
    /// Should the game automatically cast the defensive when it is ready?
    SetGameplayAutoCastDefensive,
    /// Should the game automatically cast the ultimate when it is ready?
    SetGameplayAutoCastUltimate,
    /// Change the current save game (either reset or load)
    SetGameplayChangeSaveGame,
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
pub fn setting_button<T: Resource + Component + PartialEq + Copy + std::fmt::Debug>(
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
