use bevy::prelude::*;

use crate::common::movement::MovingThingBundle;

#[derive(Component, Debug, Reflect)]
pub struct Player;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        MovingThingBundle::default(),
        SpriteBundle {
            texture: asset_server.load("wizard.png").into(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        Player,
    ));
}
