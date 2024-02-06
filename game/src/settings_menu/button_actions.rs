//! Actions that can be performed by the buttons in the settings menu.

use bevy::prelude::*;

use game_library::state::{AppState, Overlay, Settings};

use super::events::{ChangeSetting, IndividualSetting};

/// All of the various "buttons" that can be clicked in any of the main menu screens
#[derive(Component, Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub(super) enum ButtonAction {
    /// Close the menu.
    CloseMenu,
    /// Go back to the "main" settings menu.
    BackToMenu,
    /// Go to the audio menu
    SettingsAudio,
    /// Go to the display menu
    SettingsDisplay,
    /// Go to the controls menu
    SettingsControls,
    /// Go to the gameplay menu
    SettingsGameplay,
    /// Go to the accessibility menu
    SettingsAccessibility,
    /// Toggle Auto-Cast
    ToggleAutoCast,
    /// Toggle Auto-Aim
    ToggleAutoAim,
    /// Change the font family
    RotateFontFamily,
    /// Increment the main volume
    IncrementMainVolume,
    /// Increment the music volume
    IncrementMusicVolume,
    /// Increment the sound effects volume
    IncrementSoundEffectsVolume,
    /// Go to the Main Menu of the game
    GoToMainGameMenu,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct SettingsMenuButton;

/// System to handle the main menu button actions
///
/// * `interaction_query`: grabs all the buttons that have been interacted with, with the components
///    Interaction and `ButtonAction` that have a changed interaction value (i.e. the button has been
///   pressed)
/// * `app_exit_events`: can be used to send an `AppExit` event to exit the game
/// * `menu_state(next)`: lets us change the menu state for the next frame
/// * `game_state(next)`: lets us change the game state for the next frame
#[allow(clippy::type_complexity)]
pub fn menu_actions(
    interaction_query: Query<
        (&Interaction, &ButtonAction),
        (Changed<Interaction>, With<Button>, With<SettingsMenuButton>),
    >,
    mut menu_state: ResMut<NextState<Settings>>,
    mut game_state: ResMut<NextState<AppState>>,
    mut overlay_state: ResMut<NextState<Overlay>>,
    mut ew_change_setting: EventWriter<ChangeSetting>,
) {
    // Loop through all the buttons that have been interacted with
    for (interaction, menu_button_action) in &interaction_query {
        // If the button has been pressed, match the button action
        if *interaction == Interaction::Pressed {
            // Check which button action has been pressed (i.e. what action we attached to the button)
            match menu_button_action {
                ButtonAction::CloseMenu => {
                    menu_state.set(Settings::Disabled);
                    overlay_state.set(Overlay::None);
                }
                ButtonAction::GoToMainGameMenu => {
                    menu_state.set(Settings::Disabled);
                    game_state.set(AppState::MainMenu);
                }
                ButtonAction::BackToMenu => menu_state.set(Settings::Main),
                ButtonAction::SettingsAudio => menu_state.set(Settings::Audio),
                ButtonAction::SettingsDisplay => menu_state.set(Settings::Display),
                ButtonAction::SettingsControls => menu_state.set(Settings::Controls),
                ButtonAction::SettingsGameplay => menu_state.set(Settings::Gameplay),
                ButtonAction::SettingsAccessibility => menu_state.set(Settings::Accessibility),
                ButtonAction::ToggleAutoCast => {
                    ew_change_setting.send(ChangeSetting {
                        setting: IndividualSetting::AutoCast,
                    });
                }
                ButtonAction::ToggleAutoAim => {
                    ew_change_setting.send(ChangeSetting {
                        setting: IndividualSetting::AutoAim,
                    });
                }
                ButtonAction::RotateFontFamily => {
                    ew_change_setting.send(ChangeSetting {
                        setting: IndividualSetting::FontFamily,
                    });
                }
                ButtonAction::IncrementMainVolume => {
                    ew_change_setting.send(ChangeSetting {
                        setting: IndividualSetting::MainVolume,
                    });
                }
                ButtonAction::IncrementMusicVolume => {
                    ew_change_setting.send(ChangeSetting {
                        setting: IndividualSetting::MusicVolume,
                    });
                }
                ButtonAction::IncrementSoundEffectsVolume => {
                    ew_change_setting.send(ChangeSetting {
                        setting: IndividualSetting::SoundEffectsVolume,
                    });
                }
            }
        }
    }
}
