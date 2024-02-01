use bevy::prelude::*;
use game_library::{
    data_loader::TileAtlasStore, events::CastSpell, math, CursorPosition, InternalId,
    LoadedSpellData, MovementBundle, SpellBundle, SpellData, SpellLifetime, Velocity,
};

use crate::{despawn_with_tag, player::Player, resources::AppState};

use super::components::{despawn_expired_spells, SpellEntity};

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
            .insert_resource(ExistingSpells {
                data: Vec::new(),
                ids: Vec::new(),
            })
            // Load spell data (from events)
            .add_systems(Update, load_spells)
            // Spell systems
            .add_systems(
                Update,
                (despawn_expired_spells, cast_spells).run_if(in_state(AppState::InGame)),
            )
            // despawn all spells when leaving the game (to main menu)
            // stuff automatically despawns when the game exits
            .add_systems(OnEnter(AppState::MainMenu), despawn_with_tag::<SpellEntity>);
    }
}

#[derive(Resource)]
pub struct ExistingSpells {
    pub data: Vec<SpellData>,
    pub ids: Vec<String>,
}

fn load_spells(mut events: EventReader<LoadedSpellData>, mut spells: ResMut<ExistingSpells>) {
    if events.is_empty() {
        return;
    }

    tracing::info!("Load spells event with {} spells", events.len());
    for event in events.read() {
        let mut spell = event.spell_data.data.clone();
        spell.update_internal_id();
        spells.ids.push(spell.get_internal_id());
        spells.data.push(spell);
        tracing::debug!(
            "load_spells: Loaded spell {} as {}",
            event.spell_data.data.name,
            event.spell_data.data.get_internal_id()
        );
    }
}

const SPELL_SPRITE_SCALE: f32 = 0.5;
const SPELL_SPEED_MULTIPLIER: f32 = 100.0;

pub fn cast_spells(
    mut commands: Commands,
    mut event_reader: EventReader<CastSpell>,
    query: Query<&Transform, With<Player>>,
    cursor_position: Res<CursorPosition>,
    existing_spells: Res<ExistingSpells>,
    spell_atlas: Res<TileAtlasStore>,
) {
    for CastSpell(spell_identifier) in event_reader.read() {
        let Ok(player_transform) = query.get_single() else {
            tracing::error!("cast_spells: No player found, not spawning a spell");
            return;
        };

        // Todo: maybe check if the rest of the resources are ready?

        let Some(spell) = existing_spells
            .data
            .iter()
            .find(|s| &s.get_internal_id() == spell_identifier)
        else {
            tracing::error!("cast_spells: 404 {spell_identifier} not found");
            continue;
        };

        let Some(texture_atlas) = spell_atlas.tilesets.get(&spell.sprite_tileset) else {
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
