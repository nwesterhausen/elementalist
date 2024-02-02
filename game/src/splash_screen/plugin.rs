use bevy::prelude::*;

use super::{components::OnSplashScreen, scene::splash_setup, systems::countdown};
use crate::despawn_with_tag;
use game_library::state::AppState;

// This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on the state `AppState::Splash`
        app
            // When entering the state, spawn everything needed for this screen
            .add_systems(OnEnter(AppState::AppLoading), splash_setup)
            // While in this state, run the `countdown` system
            .add_systems(Update, countdown.run_if(in_state(AppState::AppLoading)))
            // When exiting the state, despawn everything that was spawned for this screen
            .add_systems(
                OnExit(AppState::AppLoading),
                despawn_with_tag::<OnSplashScreen>,
            );
    }
}
