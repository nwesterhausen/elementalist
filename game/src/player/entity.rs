use bevy::prelude::*;

use crate::common::movement::MovingThingBundle;

#[derive(Component, Debug, Reflect)]
pub struct Player;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        MovingThingBundle::default(),
        SpriteBundle {
            texture: asset_server.load("panda.png").into(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        Player,
    ));
}
