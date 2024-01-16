use bevy::prelude::*;
use game_library::StatEnum;

use crate::common::{
    movement::MovementBundle,
    stats::{Health, Mana, StatBundle},
};

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

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerBundle {
        movement: MovementBundle::default(),
        sprite: SpriteBundle {
            texture: asset_server.load("sprite/wizard.png").into(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        player: Player,
        health: Health::new(BASE_HEALTH),
        mana: Mana::new(BASE_MANA),
        stats: StatBundle::new(vec![(StatEnum::MovementSpeed, BASE_SPEED)]),
    });
}
