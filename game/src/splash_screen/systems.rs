use bevy::prelude::*;

use super::resources::SplashTimer;
use game_library::state::AppState;

// Tick the timer, and change state when finished
pub fn countdown(
    mut game_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(AppState::MainMenu);
    }
}
