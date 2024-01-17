use bevy::prelude::*;

/// Cast a spell. Sending this even will cause a spell to be cast.
#[derive(Event)]
pub struct CastSpell(pub String);
