//! Particle effect details.
use bevy::math::{Vec2, Vec4};
use bevy::reflect::Reflect;
use bevy_hanabi::prelude::*;

use crate::{colors::PaletteColor, data_loader::DataFile, enums::GameSystem, InternalId};
use std::{any::Any, fmt::Write, hash::Hash};

/// Details about a particle effect.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct Particle {
    /// The internal ID of the particle effect.
    pub internal_id: Option<String>,
    /// Color gradients for the particles.
    #[serde(default = "Vec::new")]
    pub color_gradients: Vec<ParticleColorGradient>,
    /// Size gradients for the particles.
    #[serde(default = "Vec::new")]
    pub size_gradients: Vec<ParticleSizeGradient>,
    /// The lifetime of the particles in seconds
    #[serde(default = "particle_defaults::lifetime")]
    pub lifetime: f32,
    /// The number of particles to spawn per second.
    #[serde(default = "particle_defaults::spawn_rate")]
    pub spawn_rate: f32,
    /// The maximum number of particles to be alive at any given time.
    ///
    /// Note: the lower the better
    #[serde(default = "particle_defaults::capacity")]
    pub capacity: u32,
    /// The initial position of the particles.
    #[serde(default = "ParticleInitialPosition::default")]
    pub initial_position: ParticleInitialPosition,
    /// The initial velocity of the particles.
    #[serde(default = "ParticleInitialVelocity::default")]
    pub initial_velocity: ParticleInitialVelocity,
}

mod particle_defaults {

    #[must_use]
    pub const fn lifetime() -> f32 {
        1.0
    }

    #[must_use]
    pub const fn spawn_rate() -> f32 {
        1.0
    }

    #[must_use]
    pub const fn capacity() -> u32 {
        100
    }
}

/// Initial velocity for a particle effect.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Reflect)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
pub struct ParticleInitialVelocity {
    /// initial velocity modifier
    pub center_x: f32,
    /// initial velocity modifier
    pub center_y: f32,
    /// the speed of the particles
    pub speed: f32,
}

impl Default for ParticleInitialVelocity {
    fn default() -> Self {
        Self {
            center_x: 0.0,
            center_y: 0.0,
            speed: 0.0,
        }
    }
}

/// Initial position for a particle effect.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Reflect)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
pub struct ParticleInitialPosition {
    /// initial position modifier
    pub modifier_type: PositionModifierType,
    /// the radius of the circle
    pub radius: f32,
    /// the shape dimension for spawning particles
    pub shape_dimension: ShapeDimensionType,
}

impl Default for ParticleInitialPosition {
    fn default() -> Self {
        Self {
            modifier_type: PositionModifierType::Circle,
            radius: 0.0,
            shape_dimension: ShapeDimensionType::Volume,
        }
    }
}

/// The type of position modifier for the particles.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, Reflect)]
#[serde(rename_all = "camelCase")]
pub enum PositionModifierType {
    /// A circle shape
    #[default]
    Circle,
}

/// The type of position modifier for the particles.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, Reflect)]
#[serde(rename_all = "camelCase")]
pub enum ShapeDimensionType {
    /// The entire volume of the circle
    #[default]
    Volume,
    /// Only on the edge of the circle
    Surface,
}

impl ShapeDimensionType {
    /// Convert the shape dimension type into a bevy shape dimension.
    #[must_use]
    pub const fn as_shape_dimension(&self) -> ShapeDimension {
        match self {
            Self::Volume => ShapeDimension::Volume,
            Self::Surface => ShapeDimension::Surface,
        }
    }
}

/// Color for a particle effect.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Reflect)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
pub struct ParticleColorGradient {
    /// The index of the color gradient.
    index: f32,
    /// The color of the particle.
    color: PaletteColor,
    /// The alpha value of the particle.
    #[serde(default = "default_alpha")]
    alpha: f32,
}

const fn default_alpha() -> f32 {
    1.0
}

impl Default for ParticleColorGradient {
    fn default() -> Self {
        Self {
            index: 0.0,
            color: PaletteColor::default(),
            alpha: 1.0,
        }
    }
}

/// Color for a particle effect.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Reflect)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
pub struct ParticleSizeGradient {
    /// The index of the size gradient.
    index: f32,
    /// The size of the particle.
    width: f32,
    /// The height of the particle.
    #[serde(default = "default_height")]
    height: f32,
}

const fn default_height() -> f32 {
    f32::NEG_INFINITY
}

impl Default for ParticleSizeGradient {
    fn default() -> Self {
        Self {
            index: 0.0,
            width: 1.0,
            height: 1.0,
        }
    }
}

impl Hash for Particle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.color_gradients
            .iter()
            .fold(String::new(), |mut output, b| {
                let _ = write!(output, "{:?}", b.color);
                output
            })
            .hash(state);
        self.capacity.hash(state);
    }
}

impl InternalId for Particle {
    /// Update the particle's internal ID.
    fn update_internal_id(&mut self) {
        self.internal_id = Some(self.get_internal_id());
    }
    /// Get the particle's internal ID.
    #[must_use]
    fn get_internal_id(&self) -> String {
        if self.internal_id.is_some() {
            let id = self.internal_id.clone().unwrap_or_default();
            if !id.is_empty() {
                return id;
            }
        }

        format!(
            "{}{}",
            self.color_gradients
                .iter()
                .fold(String::new(), |mut output, b| {
                    let _ = write!(output, "{:?}", b.color);
                    output
                }),
            self.capacity
        )
    }
}

impl<D: Hash + InternalId + 'static> TryInto<Particle> for DataFile<D> {
    type Error = ();

    fn try_into(self) -> Result<Particle, Self::Error> {
        if self.header.system != GameSystem::Particle {
            return Err(());
        }

        (&self.data as &dyn Any)
            .downcast_ref::<Particle>()
            .cloned()
            .ok_or(())
    }
}

impl<D: Hash + InternalId + 'static> TryFrom<&DataFile<D>> for Particle {
    type Error = ();

    fn try_from(data_file: &DataFile<D>) -> Result<Self, Self::Error> {
        if data_file.header.system != GameSystem::Particle {
            return Err(());
        }

        (&data_file.data as &dyn Any)
            .downcast_ref::<Self>()
            .cloned()
            .ok_or(())
    }
}

impl Particle {
    /// Get a gradient from the particle's color gradients.
    #[must_use]
    pub fn get_color_gradient(&self) -> Gradient<Vec4> {
        let mut gradient = Gradient::new();
        for color in &self.color_gradients {
            let mut color_vec: Vec4 = color.color.to_color().as_rgba_f32().into();
            if (color.alpha - 1.0).abs() > f32::EPSILON {
                color_vec.w = color.alpha;
            }

            gradient.add_key(color.index.clamp(0.0, 1.0), color_vec);
        }

        gradient
    }

    /// Get a gradient from the particle's size gradients.
    #[must_use]
    pub fn get_size_gradient(&self) -> Gradient<Vec2> {
        let mut gradient = Gradient::new();
        for size in &self.size_gradients {
            if size.height.is_finite() {
                gradient.add_key(
                    size.index.clamp(0.0, 1.0),
                    Vec2::new(size.width, size.height),
                );
            } else {
                gradient.add_key(
                    size.index.clamp(0.0, 1.0),
                    Vec2::new(size.width, size.width),
                );
            }
        }

        gradient
    }
}
