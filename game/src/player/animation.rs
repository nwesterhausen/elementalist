//! Animation states and systems for the player's avatar.
use bevy::prelude::*;
use elementalist_game_library::{
    events::CastSpell,
    images::{AnimationActionState, AnimationStatus},
};

use super::avatar::PlayerAvatar;

/// Set the casting animation when spell casting event is triggered.
pub(super) fn set_casting_animation(
    mut er_cast_spell: EventReader<CastSpell>,
    mut player_animation: Query<&mut AnimationStatus, With<PlayerAvatar>>,
) {
    // If the player is casting a spell, set the supplemental state to casting.
    if er_cast_spell.read().next().is_some() {
        // Expect just one result from the query
        let Ok(mut status) = player_animation.get_single_mut() else {
            tracing::warn!("set_casting_animation: Failure to get animation status for player");
            return;
        };

        status.action = AnimationActionState::Cast;
    }
}
