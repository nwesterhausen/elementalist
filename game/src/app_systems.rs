use bevy::prelude::*;

use crate::app_info;

/// Despawn all entities with the given tag (component)
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

/// Adds a TextBundle which draws the game descriptor in the top right
pub fn add_game_descriptor(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(TextBundle {
        text: Text::from_section(
            app_info::game_descriptor().as_str(),
            TextStyle {
                font: asset_server.load("ui/fonts/NanumGothicCoding-Regular.ttf"),
                font_size: 12.0,
                color: Color::rgba(1.0, 1.0, 1.0, 0.25),
            },
        ),
        style: Style {
            position_type: PositionType::Absolute,
            top: Val::Px(8.0),
            right: Val::Px(8.0),
            ..default()
        },
        ..default()
    });
}
