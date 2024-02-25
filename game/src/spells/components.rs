use bevy::prelude::*;
use game_library::spells::SpellLifetime;

/// Despawns spells when their lifetime expires, and updates their lifetime
pub fn despawn_expired_spells(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut SpellLifetime)>,
) {
    for (entity, mut lifetime) in &mut query {
        lifetime.update(time.delta_seconds());
        if lifetime.is_expired() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Component, Debug, Reflect)]
pub struct SpellEntity;
