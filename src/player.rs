use bevy::prelude::*;

/// The plugin responsible for setting up the player.
/// This is not in control of the game state, but rather just the player for the current game.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_panda);
    }
}

fn draw_panda(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("panda.png"),
        ..Default::default()
    });
}
