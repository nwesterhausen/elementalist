//! The skill book UI overlay.
use bevy::prelude::*;
use game_library::{
    colors, data_loader::storage::GameData, enums::Skill, font_resource::FontResource,
    state::Overlay,
};

pub struct SkillBookUiPlugin;

impl Plugin for SkillBookUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Overlay::SkillScreen), spawn_skill_book_ui);

        app.add_systems(Update, toggle_skill_screen);
    }
}

/// system to change system to skill screen when pressing 'k'
fn toggle_skill_screen(mut next_overlay: ResMut<NextState<Overlay>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::K) {
        next_overlay.set(Overlay::SkillScreen);
    }
}

#[derive(Component)]
pub struct SkillBlock(pub Skill);

/// Height of the skill "box"
const SKILL_HEIGHT: f32 = 80.0;
/// Width of the skill "box"
const SKILL_WIDTH: f32 = 420.0;
/// Background color of the skill "box"
const BACKGROUND_COLOR: Color = colors::KANGAROO;
/// Text color of the skill "box"
const TEXT_COLOR: Color = colors::DARKER_THUNDER;
/// Font size of the skill "box"
const FONT_SIZE: f32 = 40.0;
/// Gap between the skill "boxes"
const GAP: f32 = 10.0;
/// Border color of the skill "box"
const BORDER_COLOR: Color = colors::THUNDER;

/// Builds the single skill image
fn build_skill_image(skill: Skill, font: Handle<Font>) -> (NodeBundle, TextBundle) {
    (
        NodeBundle {
            style: Style {
                width: Val::Px(SKILL_WIDTH),
                height: Val::Px(SKILL_HEIGHT),
                margin: UiRect::bottom(Val::Px(GAP)),
                ..default()
            },
            background_color: BACKGROUND_COLOR.into(),
            border_color: BORDER_COLOR.into(),
            ..default()
        },
        TextBundle {
            text: Text::from_section(
                skill.to_string(),
                TextStyle {
                    font_size: FONT_SIZE,
                    color: TEXT_COLOR,
                    font,
                },
            ),
            style: Style {
                align_self: AlignSelf::Start,
                ..default()
            },
            ..default()
        },
    )
}

#[must_use]
fn atlas_bundle(icon_tileset: Handle<TextureAtlas>, index: usize) -> AtlasImageBundle {
    AtlasImageBundle {
        texture_atlas: icon_tileset,
        texture_atlas_image: UiTextureAtlasImage { index, ..default() },
        style: Style {
            margin: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        ..default()
    }
}

/// Builds the left column of the skill book UI.
fn build_left_column(
    font: Handle<Font>,
    icon_tileset: Handle<TextureAtlas>,
) -> Vec<((NodeBundle, TextBundle), AtlasImageBundle)> {
    let left_column = vec![
        (
            build_skill_image(Skill::Pyromancy, font.clone()),
            atlas_bundle(icon_tileset.clone(), Skill::Pyromancy.icon_index()),
        ),
        (
            build_skill_image(Skill::Fulgomancy, font.clone()),
            atlas_bundle(icon_tileset.clone(), Skill::Fulgomancy.icon_index()),
        ),
        (
            build_skill_image(Skill::Hydromancy, font.clone()),
            atlas_bundle(icon_tileset.clone(), Skill::Hydromancy.icon_index()),
        ),
        (
            build_skill_image(Skill::Geomancy, font.clone()),
            atlas_bundle(icon_tileset.clone(), Skill::Geomancy.icon_index()),
        ),
        (
            build_skill_image(Skill::Aeromancy, font.clone()),
            atlas_bundle(icon_tileset.clone(), Skill::Aeromancy.icon_index()),
        ),
        (
            build_skill_image(Skill::Cryomancy, font.clone()),
            atlas_bundle(icon_tileset.clone(), Skill::Cryomancy.icon_index()),
        ),
        (
            build_skill_image(Skill::Trudomancy, font.clone()),
            atlas_bundle(icon_tileset.clone(), Skill::Trudomancy.icon_index()),
        ),
        (
            build_skill_image(Skill::Photomancy, font.clone()),
            atlas_bundle(icon_tileset.clone(), Skill::Photomancy.icon_index()),
        ),
        (
            build_skill_image(Skill::Umbramancy, font),
            atlas_bundle(icon_tileset, Skill::Umbramancy.icon_index()),
        ),
    ];

    left_column
}

/// Builds the right column of the skill book UI.
fn build_right_column(
    font: Handle<Font>,
    icon_tileset: Handle<TextureAtlas>,
) -> Vec<((NodeBundle, TextBundle), AtlasImageBundle)> {
    let right_column = vec![
        (
            build_skill_image(Skill::Arcanomancy, font.clone()),
            atlas_bundle(icon_tileset.clone(), Skill::Arcanomancy.icon_index()),
        ),
        (
            build_skill_image(Skill::Vitomancy, font.clone()),
            atlas_bundle(icon_tileset.clone(), Skill::Vitomancy.icon_index()),
        ),
        (
            build_skill_image(Skill::Mortomancy, font.clone()),
            atlas_bundle(icon_tileset.clone(), Skill::Mortomancy.icon_index()),
        ),
        (
            build_skill_image(Skill::Ampiliomancy, font.clone()),
            atlas_bundle(icon_tileset.clone(), Skill::Ampiliomancy.icon_index()),
        ),
        (
            build_skill_image(Skill::Diminiomancy, font.clone()),
            atlas_bundle(icon_tileset.clone(), Skill::Diminiomancy.icon_index()),
        ),
        (
            build_skill_image(Skill::Citomancy, font.clone()),
            atlas_bundle(icon_tileset.clone(), Skill::Citomancy.icon_index()),
        ),
        (
            build_skill_image(Skill::Necromancy, font.clone()),
            atlas_bundle(icon_tileset.clone(), Skill::Necromancy.icon_index()),
        ),
        (
            build_skill_image(Skill::Mutatiomancy, font.clone()),
            atlas_bundle(icon_tileset.clone(), Skill::Mutatiomancy.icon_index()),
        ),
        (
            build_skill_image(Skill::Chronomancy, font),
            atlas_bundle(icon_tileset, Skill::Chronomancy.icon_index()),
        ),
    ];

    right_column
}

/// Spawns the skill book UI.
fn spawn_skill_book_ui(mut commands: Commands, fonts: Res<FontResource>, game_data: Res<GameData>) {
    let parent_container = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceAround,
            ..default()
        },
        ..default()
    };

    let left_column = NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    };
    let right_column = NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    };

    let Some(icon_tileset) = game_data.tile_atlas.get("skill_icons") else {
        tracing::error!("spawn_skill_book_ui: Failed to get skill_icons tileset");
        return;
    };

    let parent = commands.spawn(parent_container).id();
    let left = commands.spawn(left_column).id();
    let right = commands.spawn(right_column).id();

    let left_column_vec = build_left_column(fonts.interface_font.clone(), icon_tileset.clone());
    for ((node, text), icon) in left_column_vec {
        let new_entity = commands
            .spawn(node)
            .with_children(|parent| {
                parent.spawn(icon);
                parent.spawn(text);
            })
            .id();
        commands.entity(left).push_children(&[new_entity]);
    }

    let right_column_vec = build_right_column(fonts.interface_font.clone(), icon_tileset.clone());
    for ((node, text), icon) in right_column_vec {
        let new_entity = commands
            .spawn(node)
            .with_children(|parent| {
                parent.spawn(icon);
                parent.spawn(text);
            })
            .id();
        commands.entity(right).push_children(&[new_entity]);
    }

    commands.entity(parent).push_children(&[left, right]);
}
