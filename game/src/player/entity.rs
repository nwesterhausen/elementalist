use bevy::prelude::*;
use game_library::{
    colors,
    data_loader::storage::GameData,
    enums::StatEnum,
    progress_bar::{BarState, ProgressBarConfig},
    Health, Mana, MovementBundle, SpellChoices, StatBundle, Xp,
};

const fn player_base_stats(stat: &StatEnum) -> f32 {
    match stat {
        StatEnum::MovementSpeed => 100.0,
        StatEnum::Health => 10.0,
        StatEnum::Mana => 4.0,
        StatEnum::DamageReduction => 0.0,
        StatEnum::DamageResistance => 0.0,
        StatEnum::DamageReflection => 0.0,
        StatEnum::DamageAmplification => 0.0,
        StatEnum::CriticalStrikeChance => 0.0,
        StatEnum::CriticalStrikeDamage => 0.0,
        StatEnum::LifeSteal => 0.0,
        StatEnum::ManaSteal => 0.0,
        StatEnum::StunResistance => 0.0,
        StatEnum::HealthRegeneration => 0.05,
        StatEnum::ManaRegeneration => 1.0,
        StatEnum::ProjectileSpeed => 1.0,
        StatEnum::ProjectileSize => 1.0,
        StatEnum::ProjectileLifetime => 1.0,
        StatEnum::DodgeChance => 0.0,
        StatEnum::AttackDamage => 0.0,
        StatEnum::AttackSpeed => 0.0,
        StatEnum::AttackRange => 0.0,
        StatEnum::PhysicalDamageReduction => 0.0,
        StatEnum::PhysicalDamageResistance => 0.0,
        StatEnum::PhysicalDamageReflection => 0.0,
        StatEnum::PhysicalDamageAmplification => 0.0,
        StatEnum::MagicDamage => 1.0,
        StatEnum::CooldownReduction => 0.0,
        StatEnum::SpellRange => 50.0,
        StatEnum::MagicalDamageReduction => 0.0,
        StatEnum::MagicalDamageResistance => 0.0,
        StatEnum::MagicalDamageReflection => 0.0,
        StatEnum::MagicalDamageAmplification => 0.0,
    }
}

const BASE_HEALTH: u32 = 10;
const BASE_MANA: u32 = 4;

#[derive(Component, Debug, Reflect)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub movement: MovementBundle,
    pub sprite: SpriteBundle,
    pub health: Health,
    pub mana: Mana,
    pub stats: StatBundle,
    pub xp: Xp,
}

#[derive(Component, Debug, Reflect)]
pub struct PlayerAvatar;

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
        if spell_id.contains("mana-dart") {
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
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
            player: Player,
            health: Health::new(BASE_HEALTH),
            mana: Mana::new(BASE_MANA),
            stats: StatBundle::new(
                StatEnum::variants()
                    .iter()
                    .filter(|stat| *stat != &StatEnum::Health && *stat != &StatEnum::Mana)
                    .map(|stat| (stat.clone(), player_base_stats(&stat)))
                    .collect(),
            ),
            xp: Xp::default(),
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
