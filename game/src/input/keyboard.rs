use bevy::prelude::*;

use crate::AppState;

pub fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, app_state: Res<State<AppState>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match app_state.get() {
            AppState::MainMenu => {
                tracing::debug!("Escape pressed, exiting");
            }
            _ => {
                tracing::debug!("Escape pressed, going to main menu");
            }
        }
    }
}
