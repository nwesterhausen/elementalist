//! Animation states and systems for the player's avatar.
use bevy::prelude::*;
use game_library::{events::CastSpell, images::Animation};

use super::avatar::PlayerAvatar;

/// Set the casting animation when spell casting event is triggered.
pub(super) fn set_casting_animation(
    mut er_cast_spell: EventReader<CastSpell>,
    mut player_animation: Query<&mut Animation, With<PlayerAvatar>>,
) {
    // If the player is casting a spell, set the supplemental state to casting.
    if er_cast_spell.read().next().is_some() {
        for mut animation in &mut player_animation {
            match animation.clone() {
                Animation::Idle => *animation = Animation::Cast,
                Animation::Walk => *animation = Animation::WalkCast,
                Animation::Run => *animation = Animation::RunCast,
                _ => (),
            }
        }
    }
}
