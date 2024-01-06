use bevy::prelude::*;

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
pub struct OnMainMenuScreen;

// Tag component used to tag entities added on the settings menu screen
#[derive(Component)]
pub struct OnSettingsMenuScreen;

// Tag component used to tag entities added on the display settings menu screen
#[derive(Component)]
pub struct OnDisplaySettingsMenuScreen;

// Tag component used to tag entities added on the sound settings menu screen
#[derive(Component)]
pub struct OnSoundSettingsMenuScreen;

/// Tag component used to tag entities added on the controls settings menu screen
#[derive(Component)]
pub struct OnControlsSettingsMenuScreen;

/// Tag component used to tag entities added on the gameplay settings menu screen
#[derive(Component)]
pub struct OnGameplaySettingsMenuScreen;

// Tag component used to mark which setting is currently selected
#[derive(Component)]
pub struct SelectedOption;
