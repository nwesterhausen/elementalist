use bevy::prelude::*;
use game_library::{
    events::CastSpell, LoadedSpellData, MovementBundle, SpellBundle, SpellData, SpellLifetime,
    Velocity,
};

use crate::{
    common,
    player::Player,
    resources::{CursorPosition, SpellAtlas},
};

use super::components::despawn_expired_spells;

/// Spells are fired using the `CastSpell` event.
///
/// The individual spells are implemented in their own modules,
/// and will launch from the player's position when the `CastSpell`
/// event is fired.
pub struct SpellsPlugin;

impl Plugin for SpellsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CastSpell>()
            // Spell data supporting event and resources
            .add_event::<LoadedSpellData>()
            .insert_resource(ExistingSpells {
                data: Vec::new(),
                ids: Vec::new(),
            })
            // Load spell data (from events)
            .add_systems(Update, load_spells)
            // Spell systems
            .add_systems(Update, despawn_expired_spells)
            // Individual Spells
            .add_systems(Update, cast_spells);
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

pub fn cast_spells(
    mut commands: Commands,
    mut event_reader: EventReader<CastSpell>,
    query: Query<&Transform, With<Player>>,
    cursor_position: Res<CursorPosition>,
    existing_spells: Res<ExistingSpells>,
    spell_atlas: Res<SpellAtlas>,
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

        let slope_vec = common::math::slope_vec(player_transform, &cursor_position);

        // Todo: include the player's velocity in the spell's velocity
        // Todo: include the player's stats to effect the spell (damage, speed, etc)
        // Todo: figure out how we will track cooldowns. Maybe a resource?

        commands.spawn(SpellBundle {
            #[allow(clippy::cast_precision_loss)]
            lifetime: SpellLifetime::new(spell.duration as f32 / 100.0),
            movement: MovementBundle {
                #[allow(clippy::cast_precision_loss)]
                velocity: Velocity::new(slope_vec * (spell.speed as f32)),
                ..Default::default()
            },
            sprite: SpriteSheetBundle {
                texture_atlas: spell_atlas.atlas.clone(),
                sprite: spell.texture_atlas_index(),
                transform: Transform {
                    translation: player_transform.translation,
                    rotation: Quat::from_rotation_z(slope_vec.y.atan2(slope_vec.x)),
                    ..Default::default()
                },
                ..Default::default()
            },
        });
    }
}
