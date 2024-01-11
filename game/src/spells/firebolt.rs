use bevy::prelude::*;

use crate::{
    common::movement::{MovingThingBundle, Velocity},
    player::Player,
};

use super::components::{CastSpell, Spell, SpellBundle};

pub fn spawn_firebolt(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<CastSpell>,
    query: Query<&Transform, With<Player>>,
) {
    for CastSpell(spell) in event_reader.read() {
        let Ok(player_transform) = query.get_single() else {
            tracing::error!("spawn_firebolt: No player found, not spawning firebolt");
            return;
        };

        if *spell == Spell::Firebolt {
            commands.spawn(SpellBundle {
                spell: Spell::Firebolt,
                sprite: SpriteBundle {
                    texture: asset_server.load("spells/firebolt.png"),
                    transform: player_transform.clone(),
                    ..Default::default()
                },
                moving_thing: MovingThingBundle {
                    velocity: Velocity::new(Vec2::new(100.0, -2.0)),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}
