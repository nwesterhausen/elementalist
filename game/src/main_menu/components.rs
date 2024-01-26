use bevy::prelude::*;

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
pub struct OnMainMenuScreen;

/// Tag component used to mark which setting is currently selected
#[derive(Component)]
pub struct SelectedOption;
