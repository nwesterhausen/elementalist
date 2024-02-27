//! The skill book UI overlay.
use bevy::prelude::*;
use game_library::{
    colors, data_loader::storage::GameData, enums::Skill, font_resource::FontResource,
    images::StoredTextureAtlas, state::Overlay,
};

pub struct SkillBookUiPlugin;

impl Plugin for SkillBookUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Overlay::SkillScreen), spawn_skill_book_ui);

        app.add_systems(Update, toggle_skill_screen);
    }
}

/// system to change system to skill screen when pressing 'k'
fn toggle_skill_screen(
    mut next_overlay: ResMut<NextState<Overlay>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyK) {
        next_overlay.set(Overlay::SkillScreen);
    }
}

#[derive(Component)]
pub struct SkillBlock(pub Skill);

/// Height of the skill "box"
const SKILL_HEIGHT: f32 = 64.0;
/// Width of the skill "box"
const SKILL_WIDTH: f32 = 420.0;
/// Margin around the skill icon
const ICON_MARGIN: f32 = 10.0;
/// Background color of the skill "box"
const BACKGROUND_COLOR: Color = colors::SAND_DUNE;
/// Text color of the skill "box"
const TEXT_COLOR: Color = colors::DARKER_THUNDER;
/// Font size of the skill "box"
const TITLE_FONT_SIZE: f32 = 32.0;
/// Gap between the skill "boxes"
const GAP: f32 = 4.0;
/// Border color of the skill "box"
const BORDER_COLOR: Color = colors::THUNDER;
/// Skill icon background color
const SKILL_ICON_BACKGROUND: Color = colors::BRIGHT_GRAY;
/// Skill icon border width
const SKILL_ICON_BORDER_WIDTH: f32 = 2.0;
/// Skill icon border color
const SKILL_ICON_BORDER: Color = colors::ANZAC;

/// Left column skills
const LEFT_COLUMN_SKILLS: [Skill; 9] = [
    Skill::Pyromancy,
    Skill::Fulgomancy,
    Skill::Hydromancy,
    Skill::Geomancy,
    Skill::Aeromancy,
    Skill::Cryomancy,
    Skill::Trudomancy,
    Skill::Photomancy,
    Skill::Umbramancy,
];

/// Right column skills
const RIGHT_COLUMN_SKILLS: [Skill; 9] = [
    Skill::Arcanomancy,
    Skill::Vitomancy,
    Skill::Mortomancy,
    Skill::Ampiliomancy,
    Skill::Diminiomancy,
    Skill::Citomancy,
    Skill::Necromancy,
    Skill::Mutatiomancy,
    Skill::Chronomancy,
];

fn add_skill_to_ui(
    font: Handle<Font>,
    icon_tileset: StoredTextureAtlas,
    skill: Skill,
    commands: &mut Commands,
) -> Entity {
    let skill_box = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(SKILL_WIDTH),
                height: Val::Px(SKILL_HEIGHT),
                margin: UiRect::bottom(Val::Px(GAP)),
                align_content: AlignContent::Stretch,
                border: UiRect::all(Val::Px(SKILL_ICON_BORDER_WIDTH)),
                ..default()
            },
            background_color: BACKGROUND_COLOR.into(),
            border_color: BORDER_COLOR.into(),
            ..default()
        })
        .id();
    let skill_title = commands
        .spawn(TextBundle {
            text: Text::from_section(
                skill.to_string(),
                TextStyle {
                    font_size: TITLE_FONT_SIZE,
                    color: TEXT_COLOR,
                    font,
                },
            ),
            style: Style {
                align_self: AlignSelf::Start,
                ..default()
            },
            ..default()
        })
        .id();
    let skill_icon_box = commands
        .spawn(NodeBundle {
            style: Style {
                // SKILL_HEIGHT - 2 * ICON_MARGIN but rewritten for "faster" multiplication
                width: Val::Px(2.0f32.mul_add(-ICON_MARGIN, SKILL_HEIGHT)),
                height: Val::Px(2.0f32.mul_add(-ICON_MARGIN, SKILL_HEIGHT)),
                margin: UiRect::all(Val::Px(ICON_MARGIN)),
                border: UiRect::all(Val::Px(SKILL_ICON_BORDER_WIDTH)),
                padding: UiRect::top(Val::Px(2.0)),
                ..default()
            },
            background_color: SKILL_ICON_BACKGROUND.into(),
            border_color: SKILL_ICON_BORDER.into(),
            ..default()
        })
        .id();
    let skill_icon = commands
        .spawn(AtlasImageBundle {
            texture_atlas: TextureAtlas {
                layout: icon_tileset.atlas_handle.clone(),
                index: skill.icon_index(),
            },
            image: UiImage {
                texture: icon_tileset.texture_handle,
                ..default()
            },
            ..default()
        })
        .id();

    commands.entity(skill_icon_box).push_children(&[skill_icon]);
    commands
        .entity(skill_box)
        .push_children(&[skill_icon_box, skill_title]);

    skill_box
}

/// Spawns the skill book UI.
fn spawn_skill_book_ui(mut commands: Commands, fonts: Res<FontResource>, game_data: Res<GameData>) {
    let parent_container = NodeBundle {
        style: Style {
            width: Val::Percent(95.0),
            height: Val::Percent(95.0),
            justify_content: JustifyContent::SpaceAround,
            align_self: AlignSelf::Center,
            padding: UiRect::all(Val::Px(32.0)),
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

    for skill in &LEFT_COLUMN_SKILLS {
        let new_entity = add_skill_to_ui(
            fonts.interface_font.clone(),
            icon_tileset.clone(),
            *skill,
            &mut commands,
        );
        commands.entity(left).push_children(&[new_entity]);
    }

    for skill in &RIGHT_COLUMN_SKILLS {
        let new_entity = add_skill_to_ui(
            fonts.interface_font.clone(),
            icon_tileset.clone(),
            *skill,
            &mut commands,
        );
        commands.entity(right).push_children(&[new_entity]);
    }

    commands.entity(parent).push_children(&[left, right]);
}
