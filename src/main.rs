use bevy::prelude::*;

fn main() {
    App::new()
        // Load plugins
        .add_plugins((DefaultPlugins, PandaPlugin))
        // Run the app
        .run();
}

fn draw_panda(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("panda.png"),
        ..Default::default()
    });
}

pub struct PandaPlugin;

impl Plugin for PandaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_panda);
    }
}
