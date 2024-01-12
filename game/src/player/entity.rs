use bevy::prelude::*;

use crate::common::{
    movement::MovingThingBundle,
    stats::{Health, Mana, MoveSpeed, SpellDamage, SpellSpeed},
};

const BASE_SPEED: f32 = 100.0;
const BASE_HEALTH: u32 = 10;
const BASE_MANA: u32 = 4;

#[derive(Component, Debug, Reflect)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub moving_thing: MovingThingBundle,
    pub sprite: SpriteBundle,
    pub health: Health,
    pub mana: Mana,
    pub move_speed: MoveSpeed,
    pub spell_speed: SpellSpeed,
    pub spell_damage: SpellDamage,
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerBundle {
        moving_thing: MovingThingBundle::default(),
        sprite: SpriteBundle {
            texture: asset_server.load("wizard.png").into(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        player: Player,
        health: Health::new(BASE_HEALTH),
        mana: Mana::new(BASE_MANA),
        move_speed: MoveSpeed::new(BASE_SPEED),
        spell_speed: SpellSpeed::new(1.0),
        spell_damage: SpellDamage::default(),
    });
}
