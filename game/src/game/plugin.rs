use bevy::prelude::*;

use super::state::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // The game state lets us track states of the game (e.g. loading, playing, etc)
        app.add_state::<GameState>();
        // And here we could add systems dependent on the game state, such as loading/saving/changing screens.
        // Todo: add systems
    }
}
