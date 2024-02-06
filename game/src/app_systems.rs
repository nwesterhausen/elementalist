use bevy::prelude::*;
use bevy::winit::WinitWindows;
use game_library::colors;
use game_library::font_resource::FontResource;
use winit::window::Icon;

use crate::app_info;

/// Despawn all entities with the given tag (component)
pub fn despawn_with_tag<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

/// Adds a `TextBundle` which draws the game descriptor in the top right
pub fn add_game_descriptor(mut commands: Commands, fonts: Res<FontResource>) {
    commands.spawn(TextBundle {
        text: Text::from_section(
            app_info::game_descriptor().as_str(),
            TextStyle {
                font: fonts.console_font.clone(),
                font_size: 12.0,
                color: colors::TEXT_COLOR_25,
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

/// Taken directly from the bevy cheat book.
pub fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
) {
    #[cfg(not(debug_assertions))]
    let icon_path = "./assets/icon.png";
    #[cfg(debug_assertions)]
    let icon_path = "./game/assets/icon.png";

    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let Ok(image) = image::open(icon_path) else {
            tracing::warn!("set_window_icon: failed to open icon");
            return;
        };
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let Ok(icon) = Icon::from_rgba(icon_rgba, icon_width, icon_height) else {
        tracing::warn!("set_window_icon: failed to create icon");
        return;
    };

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}
