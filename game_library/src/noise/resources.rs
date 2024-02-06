use bevy::prelude::*;

/// A resource that stores the seed for the generation of the primal realm.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Resource)]
pub struct GenerationSeed(pub u64);
