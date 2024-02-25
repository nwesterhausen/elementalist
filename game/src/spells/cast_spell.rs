//! This system listens for the `CastSpell` event and spawns a spell entity based on the spell identifier.
use bevy::prelude::*;
use bevy_hanabi::{ParticleEffect, ParticleEffectBundle};
use bevy_rapier2d::prelude::*;
use game_library::{
    data_loader::storage::GameData,
    enums::ParticleAttachment,
    events::CastSpell,
    math,
    spells::{SpellBundle, SpellLifetime},
    Acceleration, CursorPosition, InternalId, Layer, MovementBundle,
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

        let Some(texture_atlas) = game_data.tile_atlas.get(&spell.sprite_tileset()) else {
            tracing::error!(
                "cast_spells: No texture atlas found for {} (spell:{})",
                spell.sprite_tileset(),
                spell.get_internal_id()
            );
            continue;
        };

        // To know where to "aim" the spell, we need to calculate the slope between the player and the cursor
        let slope_vec = math::slope_vec(player_transform, &cursor_position);

        // Todo: include the player's velocity in the spell's velocity
        // Todo: include the player's stats to effect the spell (damage, speed, etc)
        // Todo: figure out how we will track cooldowns. Maybe a resource?

        let spell_projectile = commands
            .spawn((
                SpellBundle {
                    lifetime: SpellLifetime::new(spell.duration()),
                    movement: MovementBundle {
                        velocity: Velocity {
                            linvel: slope_vec * (spell.speed() * SPELL_SPEED_MULTIPLIER),
                            ..default()
                        },
                        acceleration: Acceleration::new(slope_vec * spell.acceleration()),
                    },
                    sprite: SpriteSheetBundle {
                        texture_atlas: texture_atlas.clone(),
                        sprite: TextureAtlasSprite::new(spell.sprite_tileset_index()),
                        transform: Transform {
                            translation: player_transform.translation - Vec3::new(0.0, 0.0, 0.1),
                            rotation: Quat::from_rotation_z(slope_vec.y.atan2(slope_vec.x)),
                            scale: Vec3::splat(SPELL_SPRITE_SCALE),
                        },
                        ..Default::default()
                    },
                },
                SpellEntity,
                RigidBody::KinematicVelocityBased,
                Collider::ball(4.0),
                Layer::Foreground(10),
            ))
            .id();

        // check for any particles that go on the projectile
        let projectile_particles = spell
            .particles()
            .iter()
            .filter_map(|particle_link| {
                if particle_link.attachment == ParticleAttachment::Projectile {
                    tracing::info!("Spawning projectile particle {}", particle_link.particle_id);
                    game_data.particles.get(&particle_link.particle_id)
                } else {
                    None
                }
            })
            .map(|particle| {
                commands
                    .spawn(ParticleEffectBundle {
                        effect: ParticleEffect::new(particle.clone()),
                        transform: Transform {
                            translation: Vec3::ZERO,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .id()
            })
            .collect::<Vec<Entity>>();

        // insert the particles into the spell entity
        commands
            .entity(spell_projectile)
            .push_children(&projectile_particles);
    }
}
