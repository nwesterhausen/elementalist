use bevy::prelude::*;
use game_library::events::CastSpell;
use leafwing_input_manager::action_state::ActionState;

use crate::{events::PlayerAction, player::Player, resources::SpellChoices};

/// Handle player input
pub fn player_control_system(
    query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut ew_cast_spell: EventWriter<CastSpell>,
    spell_choices: Res<SpellChoices>,
) {
    let action_state = query.single();
    if action_state.pressed(PlayerAction::Look) {
        if let Some(axis_pair) = action_state.clamped_axis_pair(PlayerAction::Look) {
            println!("Look: {axis_pair:?}");
        } else {
            println!("Look");
        }
    }
    if action_state.just_pressed(PlayerAction::CastPrimary) {
        // Cast a the primary spell
        if let Some(spell_id) = spell_choices.primary.clone() {
            ew_cast_spell.send(CastSpell(spell_id));
        } else {
            tracing::warn!("No primary spell selected");
        }
    }
    if action_state.just_pressed(PlayerAction::CastSecondary) {
        // Cast a the secondary spell
        if let Some(spell_id) = spell_choices.secondary.clone() {
            ew_cast_spell.send(CastSpell(spell_id));
        } else {
            tracing::warn!("No secondary spell selected");
        }
    }
    if action_state.just_pressed(PlayerAction::CastDefensive) {
        // Cast a the defensive spell
        if let Some(spell_id) = spell_choices.defensive.clone() {
            ew_cast_spell.send(CastSpell(spell_id));
        } else {
            tracing::warn!("No defensive spell selected");
        }
    }
    if action_state.just_pressed(PlayerAction::CastUltimate) {
        // Cast a the ultimate spell
        if let Some(spell_id) = spell_choices.ultimate.clone() {
            ew_cast_spell.send(CastSpell(spell_id));
        } else {
            tracing::warn!("No ultimate spell selected");
        }
    }
    if action_state.just_pressed(PlayerAction::ToggleAutoCast) {
        println!("ToggleAutoCast");
    }
    if action_state.just_pressed(PlayerAction::ToggleAutoAim) {
        println!("ToggleAutoAim");
    }
    if action_state.just_pressed(PlayerAction::Interact) {
        println!("Interact");
    }
}
