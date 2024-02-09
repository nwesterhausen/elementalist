use bevy::prelude::*;
use game_library::{
    colors,
    data_loader::storage::GameData,
    enums::StatEnum,
    progress_bar::{BarState, ProgressBarConfig},
    Health, Mana, MovementBundle, SpellChoices, StatBundle, Xp,
};

/// Base stats for the player. These are the stats that the player starts with, and are used to
/// initiate the [`StatBundle`] for the player.
const fn player_base_stats(stat: &StatEnum) -> f32 {
    match stat {
        StatEnum::MovementSpeed => 100.0,
        StatEnum::SpellRange => 50.0,
        StatEnum::Health => 10.0,
        StatEnum::Mana => 4.0,
        StatEnum::ManaRegeneration
        | StatEnum::ProjectileSpeed
        | StatEnum::ProjectileSize
        | StatEnum::ProjectileLifetime
        | StatEnum::MagicDamage => 1.0,
        StatEnum::HealthRegeneration => 0.05,
        StatEnum::DamageReduction
        | StatEnum::DamageResistance
        | StatEnum::DamageReflection
        | StatEnum::DamageAmplification
        | StatEnum::CriticalStrikeChance
        | StatEnum::CriticalStrikeDamage
        | StatEnum::LifeSteal
        | StatEnum::ManaSteal
        | StatEnum::StunResistance
        | StatEnum::DodgeChance
        | StatEnum::AttackDamage
        | StatEnum::AttackSpeed
        | StatEnum::AttackRange
        | StatEnum::PhysicalDamageReduction
        | StatEnum::PhysicalDamageResistance
        | StatEnum::PhysicalDamageReflection
        | StatEnum::PhysicalDamageAmplification
        | StatEnum::CooldownReduction
        | StatEnum::MagicalDamageReduction
        | StatEnum::MagicalDamageResistance
        | StatEnum::MagicalDamageReflection
        | StatEnum::MagicalDamageAmplification => 0.0,
    }
}

/// Base health for the player. Specified outside of `base_stats` because it's an integer.
const BASE_HEALTH: u32 = 10;
/// Base mana for the player. Specified outside of `base_stats` because it's an integer.
const BASE_MANA: u32 = 4;

/// Player is a marker component for the player. This is used to identify the player's entity
/// in queries and systems. This should be used instead of using the `PlayerAvatar` component
/// directly, as it allows for more flexibility in the future.
#[derive(Component, Debug, Reflect)]
pub struct Player;

/// `PlayerBundle` is a bundle of components that are used to create the player's entity. This is
/// used to spawn the player's entity in the game. They are a set of components that are integral
/// to the player and we want to ensure that they are always present when the player is spawned.
///
/// These are specifically related to spawning the [`PlayerAvatar`], and are used to track the
/// player's health, mana, stats, and experience points while in a game run. The overall player
/// skills and unlocked spells are tracked in another resource (`todo!()`).
#[derive(Bundle)]
pub struct PlayerBundle {
    /// The player's movement bundle.
    pub movement: MovementBundle,
    /// The player's sprite bundle.
    pub sprite: SpriteBundle,
    /// The player's health.
    pub health: Health,
    /// The player's mana.
    pub mana: Mana,
    /// The player's stats.
    pub stats: StatBundle,
    /// The player's experience points.
    pub xp: Xp,
    /// Player marker component.
    pub player: Player,
}

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
    asset_server: Res<AssetServer>,
    mut spell_choices: ResMut<SpellChoices>,
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

    commands.spawn((
        PlayerBundle {
            movement: MovementBundle::default(),
            sprite: SpriteBundle {
                texture: asset_server.load("sprite/wizard.png"),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 50.0),
                    scale: Vec3::splat(0.75),
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
