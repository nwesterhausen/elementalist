//! This system listens for the `CastSpell` event and spawns a spell entity based on the spell identifier.
use bevy::prelude::*;
use game_library::{
    data_loader::storage::GameData, events::CastSpell, math, CursorPosition, InternalId,
    MovementBundle, SpellBundle, SpellLifetime, Velocity,
};

use crate::player::Player;

use super::components::SpellEntity;

const SPELL_SPRITE_SCALE: f32 = 0.5;
const SPELL_SPEED_MULTIPLIER: f32 = 100.0;

pub(super) fn cast_spells(
    mut commands: Commands,
    mut event_reader: EventReader<CastSpell>,
    query: Query<&Transform, With<Player>>,
    cursor_position: Res<CursorPosition>,
    game_data: Res<GameData>,
) {
    for CastSpell(spell_identifier) in event_reader.read() {
        let Ok(player_transform) = query.get_single() else {
            tracing::error!("cast_spells: No player found, not spawning a spell");
            return;
        };

        let Some(spell) = game_data.spells.get(spell_identifier) else {
            tracing::error!("cast_spells: 404 {spell_identifier} not found");
            continue;
        };

        let Some(texture_atlas) = game_data.tile_atlas.get(&spell.sprite_tileset) else {
            tracing::error!(
                "cast_spells: No texture atlas found for {} (spell:{})",
                spell.sprite_tileset,
                spell.get_internal_id()
            );
            continue;
        };

        let slope_vec = math::slope_vec(player_transform, &cursor_position);

        // Todo: include the player's velocity in the spell's velocity
        // Todo: include the player's stats to effect the spell (damage, speed, etc)
        // Todo: figure out how we will track cooldowns. Maybe a resource?

        commands.spawn((
            SpellBundle {
                lifetime: SpellLifetime::new(spell.duration),
                movement: MovementBundle {
                    velocity: Velocity::new(slope_vec * (spell.speed * SPELL_SPEED_MULTIPLIER)),
                    ..Default::default()
                },
                sprite: SpriteSheetBundle {
                    texture_atlas: texture_atlas.clone(),
                    sprite: spell.texture_atlas_index(),
                    transform: Transform {
                        translation: player_transform.translation - Vec3::new(0.0, 0.0, 0.1),
                        rotation: Quat::from_rotation_z(slope_vec.y.atan2(slope_vec.x)),
                        scale: Vec3::splat(SPELL_SPRITE_SCALE),
                    },
                    ..Default::default()
                },
            },
            SpellEntity,
        ));
    }
}
