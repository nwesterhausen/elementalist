use bevy::prelude::*;
use elementalist_game_library::{
    events::CastSpell,
    settings::GameplaySettings,
    spells::SpellSelection,
    state::{AppState, Overlay},
};
use leafwing_input_manager::action_state::ActionState;

use crate::{events::PlayerAction, player::Player};

pub(super) struct PlayerControlsPlugin;

impl Plugin for PlayerControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_press_pause_system,
                player_toggle_auto_system,
                player_cast_primary_spell_system,
                player_cast_secondary_spell_system,
                player_cast_defensive_spell_system,
                player_cast_ultimate_spell_system,
                player_control_system,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}

/// System to handle player casting primary spell
fn player_cast_primary_spell_system(
    mut ew_cast_spell: EventWriter<CastSpell>,
    spell_choices: Res<SpellSelection>,
    query: Query<&ActionState<PlayerAction>, With<Player>>,
) {
    let Ok(action_state) = query.get_single() else {
        tracing::warn!("cast_primary: Failure to get action state for player");
        return;
    };

    if action_state.just_pressed(&PlayerAction::CastPrimary) {
        // Cast a the primary spell
        if let Some(spell_id) = spell_choices.primary() {
            ew_cast_spell.send(CastSpell(spell_id));
        } else {
            tracing::warn!("No primary spell selected");
        }
    }
}

/// System to handle player casting secondary spell
fn player_cast_secondary_spell_system(
    mut ew_cast_spell: EventWriter<CastSpell>,
    spell_choices: Res<SpellSelection>,
    query: Query<&ActionState<PlayerAction>, With<Player>>,
) {
    let Ok(action_state) = query.get_single() else {
        tracing::warn!("cast_secondary: Failure to get action state for player");
        return;
    };

    if action_state.just_pressed(&PlayerAction::CastSecondary) {
        // Cast a the secondary spell
        if let Some(spell_id) = spell_choices.secondary() {
            ew_cast_spell.send(CastSpell(spell_id));
        } else {
            tracing::warn!("No secondary spell selected");
        }
    }
}

/// System to handle player casting defensive spell
fn player_cast_defensive_spell_system(
    mut ew_cast_spell: EventWriter<CastSpell>,
    spell_choices: Res<SpellSelection>,
    query: Query<&ActionState<PlayerAction>, With<Player>>,
) {
    let Ok(action_state) = query.get_single() else {
        tracing::warn!("cast_defensive: Failure to get action state for player");
        return;
    };

    if action_state.just_pressed(&PlayerAction::CastDefensive) {
        // Cast a the defensive spell
        if let Some(spell_id) = spell_choices.tertiary() {
            ew_cast_spell.send(CastSpell(spell_id));
        } else {
            tracing::warn!("No defensive spell selected");
        }
    }
}

/// System to handle player casting ultimate spell
fn player_cast_ultimate_spell_system(
    mut ew_cast_spell: EventWriter<CastSpell>,
    spell_choices: Res<SpellSelection>,
    query: Query<&ActionState<PlayerAction>, With<Player>>,
) {
    let Ok(action_state) = query.get_single() else {
        tracing::warn!("cast_ultimate: Failure to get action state for player");
        return;
    };

    if action_state.just_pressed(&PlayerAction::CastUltimate) {
        // Cast a the ultimate spell
        if let Some(spell_id) = spell_choices.ultimate() {
            ew_cast_spell.send(CastSpell(spell_id));
        } else {
            tracing::warn!("No ultimate spell selected");
        }
    }
}

/// Handle player input
fn player_control_system(query: Query<&ActionState<PlayerAction>, With<Player>>) {
    let Ok(action_state) = query.get_single() else {
        tracing::warn!("control: Failure to get action state for player");
        return;
    };

    if action_state.pressed(&PlayerAction::Look) {
        if let Some(axis_pair) = action_state.clamped_axis_pair(&PlayerAction::Look) {
            println!("Look: {axis_pair:?}");
        } else {
            println!("Look");
        }
    }
    if action_state.just_pressed(&PlayerAction::Interact) {
        println!("Interact");
    }
}

/// Handle player toggling auto-cast or auto-aim
fn player_toggle_auto_system(
    query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut gameplay_settings: ResMut<GameplaySettings>,
) {
    let Ok(action_state) = query.get_single() else {
        tracing::warn!("toggle_auto: Failure to get action state for player");
        return;
    };

    if action_state.just_pressed(&PlayerAction::ToggleAutoCast) {
        gameplay_settings.auto_cast = !gameplay_settings.auto_cast;
    }
    if action_state.just_pressed(&PlayerAction::ToggleAutoAim) {
        gameplay_settings.auto_aim = !gameplay_settings.auto_aim;
    }
}

/// Handle player pressing pause button
fn player_press_pause_system(
    query: Query<&ActionState<PlayerAction>, With<Player>>,
    current_state: Res<State<AppState>>,
    mut next_overlay_state: ResMut<NextState<Overlay>>,
) {
    if *current_state.get() != AppState::InGame {
        return;
    }

    let Ok(action_state) = query.get_single() else {
        tracing::warn!("press_pause: Failure to get action state for player");
        return;
    };

    if action_state.just_pressed(&PlayerAction::Pause) {
        // Pause the game (automatically happens when in [`Overlay::Settings`]) and open the settings menu
        next_overlay_state.set(Overlay::Settings);
    }
}
