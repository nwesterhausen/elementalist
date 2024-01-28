use bevy::prelude::*;

pub(super) struct LoadSpritesheetPlugin;

impl Plugin for LoadSpritesheetPlugin {
    fn build(&self, app: &mut App) {
        // Add the resources
        app.init_resource::<SpellAtlas>()
            .init_resource::<SkillAtlas>();
        // Add the systems
        app.add_systems(Startup, (load_spell_atlas, load_skill_atlas));
    }
}

#[derive(Resource, Debug, Default, Reflect)]
pub struct SpellAtlas {
    pub atlas: Handle<TextureAtlas>,
}

fn load_spell_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("tileset/spell_projectiles.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 5, 6, None, None);
    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(SpellAtlas {
        atlas: atlas_handle,
    });
}

#[derive(Resource, Debug, Reflect, Default)]
pub struct SkillAtlas {
    pub atlas: Handle<TextureAtlas>,
}

fn load_skill_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("tileset/skill_icons.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 5, 6, None, None);
    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(SkillAtlas {
        atlas: atlas_handle,
    });
}
