use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct SpellAtlas {
    pub atlas: Handle<TextureAtlas>,
}

pub fn load_spell_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("tileset/spell_projectiles.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 5, 5, None, None);
    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(SpellAtlas {
        atlas: atlas_handle,
    });
}
