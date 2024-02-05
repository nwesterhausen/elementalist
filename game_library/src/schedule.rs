//! Provides sets to use with any system to provide consistent ordering.

use bevy::prelude::*;

/// Provides sets to use with any system to provide consistent ordering.
///
/// The ordering is configured in the `SchedulingPlugin` and by putting systems within
/// one of these sets will ensure that they are run in the correct order.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum GameSet {
    /// User input systems.
    UserInput,
    /// Entity update systems.
    ///
    /// This set is for systems that update entities in the game world, e.g. movement,
    /// status effects, etc.
    EntityUpdate,
    /// Collision systems.
    ///
    /// This set is for systems that handle collision detection and tagging (for specific
    /// resolution either in a despawn system or entity update system).
    Collision,
    /// Despawn systems.
    ///
    /// This set is for systems that handle despawning entities from the game world.
    Despawn,
}

/// Plugin to configure the scheduling of systems.
pub struct SchedulingPlugin;

impl Plugin for SchedulingPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                GameSet::Despawn,
                // We flush before the next..
                GameSet::UserInput,
                GameSet::EntityUpdate,
                GameSet::Collision,
            )
                .chain(),
        )
        .add_systems(
            Update,
            apply_deferred
                .after(GameSet::Despawn)
                .before(GameSet::UserInput),
        );
    }
}
