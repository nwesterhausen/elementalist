use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use game_library::{
    colors,
    data_loader::storage::GameData,
    enums::StatEnum,
    images::AnimationBundle,
    progress_bar::{BarState, ProgressBarConfig},
    spells::SpellSelection,
    Health, Layer, Mana, MovementBundle, StatBundle, Xp,
};

use super::{
    bundle::{player_base_stats, PlayerBundle, BASE_HEALTH, BASE_MANA},
    Player,
};

const INITIAL_SPRITE_INDEX: usize = 3;

/// `PlayerAvatar` is a marker component for the player's avatar, i.e. the entity that the player
/// controls. This is used to ensure that only one player avatar is spawned at a time. It also
/// lets us allow a player controller to continue to control the menu and other things without
/// having a sprite on screen or something like that.
#[derive(Component, Debug, Reflect)]
pub struct PlayerAvatar;

/// Spawns the player avatar if there isn't one already.
///
/// This system is used to spawn the player's avatar in the game. It checks if there is already
/// a player avatar in the game, and if there isn't, it spawns one. It also sets the player's
/// primary and secondary spells to the first two spells that are found in the game data.
///
/// This will put the avatar as a child of the player entity, which should exist already.
pub fn spawn_player_avatar(
    mut commands: Commands,
    mut spell_choices: ResMut<SpellSelection>,
    game_data: Res<GameData>,
    existing_players: Query<&PlayerAvatar>,
) {
    // Only spawn player if there isn't one already
    if existing_players.iter().count() > 0 {
        return;
    }

    // Load spells (forced right now)
    for spell_id in game_data.spells.iter_ids() {
        if spell_id.contains("time_dart") {
            spell_choices.set_primary_by_id(spell_id.clone());
        }
        if spell_id.contains("spark") {
            spell_choices.set_secondary_by_id(spell_id.clone());
        }
    }

    // Load wizard tileset
    let Some(tileset) = game_data.tile_atlas.get("wizard") else {
        error!("Failed to load wizard tileset");
        return;
    };

    commands.spawn((
        PlayerBundle {
            animation: AnimationBundle::new("player-avatar"),
            movement: MovementBundle::default(),
            sprite: SpriteSheetBundle {
                atlas: TextureAtlas {
                    layout: tileset.atlas_handle.clone(),
                    index: INITIAL_SPRITE_INDEX,
                },
                texture: tileset.texture_handle.clone(),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 150.0),
                    scale: Vec3::splat(0.85),
                    ..Default::default()
                },
                ..Default::default()
            },
            health: Health::new(BASE_HEALTH),
            mana: Mana::new(BASE_MANA),
            stats: StatBundle::new(
                StatEnum::variants()
                    .iter()
                    .filter(|stat| *stat != &StatEnum::Health && *stat != &StatEnum::Mana)
                    .map(|stat| (stat.clone(), player_base_stats(stat)))
                    .collect(),
            ),
            xp: Xp::default(),
            player: Player,
            kinematic_controller: KinematicCharacterController::default(),
            collider: Collider::capsule_y(6.0, 4.0),
            rigid_body: RigidBody::KinematicVelocityBased,
            layer: Layer::Foreground(i16::MAX),
        },
        ProgressBarConfig::<Xp>::default()
            .with_background_color(colors::BACKGROUND_COLOR_50)
            .with_single_color(colors::SKILL_TRACK_BAR_COLOR)
            .with_size((10.0, 2.0).into())
            .with_position_translation(Vec3::new(-5.0, 26.0, 0.0)),
        ProgressBarConfig::<Health>::default()
            .with_background_color(colors::BACKGROUND_COLOR_50)
            .with_color(&BarState::Ok, colors::HEALTH_BAR_COLOR_OK)
            .with_color(&BarState::Moderate, colors::HEALTH_BAR_COLOR_MODERATE)
            .with_color(&BarState::Critical, colors::HEALTH_BAR_COLOR_CRITICAL)
            .with_size((10.0, 2.0).into())
            .with_position_translation(Vec3::new(-5.0, 24.0, 0.0)),
        ProgressBarConfig::<Mana>::default()
            .with_background_color(colors::BACKGROUND_COLOR_50)
            .with_single_color(colors::MANA_BAR_COLOR)
            .with_size((10.0, 2.0).into())
            .with_position_translation(Vec3::new(-5.0, 22.0, 0.0)),
        PlayerAvatar,
    ));
}
