use bevy::prelude::*;
use game_library::{enums::StatEnum, Health, Mana, MovementBundle, SpellChoices, StatBundle};

use crate::spells::ExistingSpells;

const BASE_SPEED: f32 = 100.0;
const BASE_HEALTH: u32 = 10;
const BASE_MANA: u32 = 4;

#[derive(Component, Debug, Reflect)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub movement: MovementBundle,
    pub sprite: SpriteBundle,
    pub health: Health,
    pub mana: Mana,
    pub stats: StatBundle,
}

#[derive(Component, Debug, Reflect)]
pub struct PlayerAvatar;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spell_choices: ResMut<SpellChoices>,
    loaded_spells: Res<ExistingSpells>,
    existing_players: Query<&PlayerAvatar>,
) {
    // Only spawn player if there isn't one already
    if existing_players.iter().count() > 0 {
        return;
    }

    // Load spells (forced right now)
    for spell_id in &loaded_spells.ids {
        if spell_id.contains("WaterBolt") {
            spell_choices.set_primary_by_id(spell_id.clone());
        }
        if spell_id.contains("Spark") {
            spell_choices.set_secondary_by_id(spell_id.clone());
        }
    }

    commands.spawn((
        PlayerBundle {
            movement: MovementBundle::default(),
            sprite: SpriteBundle {
                texture: asset_server.load("sprite/wizard.png"),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
            player: Player,
            health: Health::new(BASE_HEALTH),
            mana: Mana::new(BASE_MANA),
            stats: StatBundle::new(vec![(StatEnum::MovementSpeed, BASE_SPEED)]),
        },
        PlayerAvatar,
    ));
}
