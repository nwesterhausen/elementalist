use bevy::prelude::*;

use super::{
    components::{despawn_expired_spells, CastSpell},
    firebolt,
};

/// Spells are fired using the `CastSpell` event.
///
/// The individual spells are implemented in their own modules,
/// and will launch from the player's position when the `CastSpell`
/// event is fired.
pub struct SpellsPlugin;

impl Plugin for SpellsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CastSpell>()
            // Spell systems
            .add_systems(Update, despawn_expired_spells)
            // Individual Spells
            .add_systems(Update, firebolt::spawn_firebolt);
    }
}
