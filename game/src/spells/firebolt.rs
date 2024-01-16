use bevy::prelude::*;

use crate::{
    common::{
        math,
        movement::{MovementBundle, Velocity},
    },
    player::Player,
    resources::CursorPosition,
};
use game_library::Spell;

use super::components::{CastSpell, SpellBundle, SpellLifetime};

const FIREBOLT_SPEED: f32 = 1000.0;
const FIREBOLT_LIFETIME: f32 = 1.0;

pub fn spawn_firebolt(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<CastSpell>,
    query: Query<&Transform, With<Player>>,
    cursor_position: Res<CursorPosition>,
) {
    for CastSpell(spell) in event_reader.read() {
        let Ok(player_transform) = query.get_single() else {
            tracing::error!("spawn_firebolt: No player found, not spawning firebolt");
            return;
        };

        if *spell == Spell::Firebolt {
            let slope_vec = math::slope_vec(&player_transform, &cursor_position);

            commands.spawn(SpellBundle {
                spell: Spell::Firebolt,
                lifetime: SpellLifetime::new(FIREBOLT_LIFETIME),
                sprite: SpriteBundle {
                    texture: asset_server.load("spells/firebolt.png"),
                    transform: player_transform.clone(),
                    ..Default::default()
                },
                movement: MovementBundle {
                    velocity: Velocity::new(slope_vec * FIREBOLT_SPEED),
                    ..Default::default()
                },
            });
        }
    }
}