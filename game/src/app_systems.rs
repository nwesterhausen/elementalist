use bevy::prelude::*;
use bevy::winit::WinitWindows;
use winit::window::Icon;

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
        let image = image::open(icon_path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}

/// Reads game_data and imports the data into the game
pub use game_library::data_loader::load_data_file_dir;
