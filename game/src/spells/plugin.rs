use bevy::prelude::*;

use super::{components::CastSpell, firebolt};

pub struct SpellsPlugin;

impl Plugin for SpellsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CastSpell>()
            .add_systems(Update, firebolt::spawn_firebolt);
    }
}
