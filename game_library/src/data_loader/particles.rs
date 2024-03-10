//! Loads particles from the data files and stores them in the particle effect store.
use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use crate::{enums::GameSystem, particle::Particle};

use super::{
    events::{DataFileFound, LoadedParticleData},
    loader::read_data_file,
    storage::GameData,
    DataFile,
};

/// System to load a particle effect.
pub(super) fn load_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut er_particle_df: EventReader<LoadedParticleData>,
    mut game_data: ResMut<GameData>,
) {
    for data_file in er_particle_df.read() {
        let unique_id = &data_file.particle_data.header.unique_id;
        let particle = &data_file.particle_data.data;

        let writer = ExprWriter::new();

        let age = writer.lit(0.).expr();
        let init_age = SetAttributeModifier::new(Attribute::AGE, age);

        let lifetime = writer.lit(particle.lifetime).expr();
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        let init_pos = SetPositionCircleModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            axis: writer.lit(Vec3::Z).expr(),
            radius: writer.lit(particle.initial_position.radius).expr(),
            dimension: particle
                .initial_position
                .shape_dimension
                .as_shape_dimension(),
        };

        let init_vel = SetVelocityCircleModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            axis: writer.lit(Vec3::Z).expr(),
            speed: writer.lit(particle.initial_velocity.speed).expr(),
        };

        let spawner = Spawner::rate(particle.spawn_rate.into());
        let effect = effects.add(
            EffectAsset::new(particle.capacity, spawner, writer.finish())
                .with_name(unique_id)
                .init(init_pos)
                .init(init_vel)
                .init(init_age)
                .init(init_lifetime)
                .render(SizeOverLifetimeModifier {
                    gradient: particle.get_size_gradient(),
                    screen_space_size: false,
                })
                .render(ColorOverLifetimeModifier {
                    gradient: particle.get_color_gradient(),
                }),
        );

        game_data.particles.insert(String::from(unique_id), effect);
    }
}

/// System to parse a particle data file.
pub(super) fn parse_particle_file(
    mut er_df_found: EventReader<DataFileFound>,
    mut ew_particle_df: EventWriter<LoadedParticleData>,
) {
    for event in er_df_found.read() {
        if event.header.system == GameSystem::Particle {
            let particle_data: DataFile<Particle> = if let Some(d) = read_data_file(&event.filepath)
            {
                d
            } else {
                warn!(
                    "load_data_file_dir: failed to read particle data from {}",
                    event.header.unique_id
                );
                continue;
            };
            ew_particle_df.send(LoadedParticleData { particle_data });
        }
    }
}
