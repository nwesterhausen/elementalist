use crate::despawn_with_tag;
use bevy::prelude::*;
use elementalist_game_library::state::AppState;
use elementalist_game_library::{events::CastSpell, state::Overlay};

use super::{
    cast_spell::cast_spells,
    components::{despawn_expired_spells, SpellEntity},
};

/// Spells are fired using the `CastSpell` event.
///
/// The individual spells are implemented in their own modules,
/// and will launch from the player's position when the `CastSpell`
/// event is fired.
pub struct SpellsPlugin;

impl Plugin for SpellsPlugin {
    fn build(&self, app: &mut App) {
        // Spell data supporting event and resources
        app.add_event::<CastSpell>()
            // Spell systems
            .add_systems(
                Update,
                (despawn_expired_spells, cast_spells)
                    .run_if(in_state(AppState::InGame).and_then(not(in_state(Overlay::Settings)))),
            )
            // despawn all spells when leaving the game (to main menu)
            // stuff automatically despawns when the game exits
            .add_systems(OnExit(AppState::InGame), despawn_with_tag::<SpellEntity>);
    }
}
