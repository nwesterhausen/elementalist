//! This may end up a full module. It is the plugin/systems for the status screen overlay.
use bevy::prelude::*;
use game_library::{
    font_resource::FontResource,
    state::{AppState, Overlay},
    StatBundle,
};
use leafwing_input_manager::action_state::ActionState;

use crate::{despawn_with_tag, events::PlayerAction, player::Player, style_prefab};

pub(super) struct StatusScreenPlugin;

impl Plugin for StatusScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Overlay::Status), spawn_status_screen);
        app.add_systems(
            Update,
            toggle_status_screen.run_if(in_state(AppState::InGame)),
        );
        app.add_systems(
            OnExit(Overlay::Status),
            despawn_with_tag::<StatusScreenEntity>,
        );
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
struct StatusScreenEntity;

/// System to setup the status screen
fn spawn_status_screen(
    mut commands: Commands,
    fonts: Res<FontResource>,
    stat_query: Query<&StatBundle, With<Player>>,
) {
    let overlay_parent = commands
        .spawn((
            style_prefab::main_menu_full_node_bundle(),
            StatusScreenEntity,
        ))
        .with_children(|parent| {
            // Status Title
            parent.spawn(style_prefab::main_menu_title_bundle(
                "Status",
                fonts.display_font.clone(),
            ));
        })
        .id();

    // There should be a single stat bundle for the player
    let Ok(stat_bundle) = stat_query.get_single() else {
        tracing::error!("spawn_status_screen: Failure to get stat bundle for player");
        return;
    };

    let stat_parent = commands
        .spawn(style_prefab::status_screen_stats_node_bundle())
        .id();

    let new_children: Vec<Entity> = stat_bundle
        .stats
        .iter()
        .map(|(stat, value)| {
            return commands
                .spawn((
                    style_prefab::status_screen_text_bundle(
                        format!("{stat}: {value}"),
                        fonts.console_font.clone(),
                    ),
                    StatusScreenEntity,
                ))
                .id();
        })
        .collect();

    commands
        .entity(overlay_parent)
        .push_children(&[stat_parent]);
    commands.entity(stat_parent).push_children(&new_children);
}

/// System to handle opening the status screen or closing it
fn toggle_status_screen(
    mut next_state: ResMut<NextState<Overlay>>,
    state: Res<State<Overlay>>,
    query: Query<&ActionState<PlayerAction>, With<Player>>,
) {
    if let Ok(action_state) = query.get_single() {
        if action_state.just_pressed(&PlayerAction::StatusOverlay) {
            if *state == Overlay::Status {
                next_state.set(Overlay::None);
            } else {
                next_state.set(Overlay::Status);
            }
        }
    } else {
        tracing::warn!("toggle_status_screen: Failure to get action state for player");
    }
}
