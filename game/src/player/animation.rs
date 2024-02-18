//! Animation states and systems for the player's avatar.
use bevy::prelude::*;
use game_library::events::CastSpell;

use super::avatar::PlayerAvatar;

/// The different states that the player's avatar can be in.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub(super) enum PlayerAnimation {
    /// The player's avatar is idle.
    #[default]
    Idle,
    /// The player's avatar is walking.
    Walking,
}

/// Supplemental animations states for the player's avatar.
///
/// These can be combined with the `PlayerAnimation` states to create more complex animations.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub(super) enum PlayerAnimationSupplemental {
    /// None
    #[default]
    None,
    /// Casting a spell
    Casting,
}

/// Facing direction of the player's avatar.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub(super) enum PlayerFacing {
    /// The player's avatar is facing left.
    #[default]
    Left,
    /// The player's avatar is facing right.
    Right,
}

/// Timer that will be used to update the player's avatar's animation.
#[derive(Debug, Default, Resource)]
pub(super) struct PlayerAnimationTimer(pub Timer);

const WALKING_ANIMATION: [usize; 4] = [0, 1, 0, 2];
const CASTING_WALKING_ANIMATION: [usize; 4] = [5, 6, 7, 8];
const IDLE_ANIMATION: [usize; 1] = [3];
const CASTING_IDLE_ANIMATION: [usize; 1] = [4];

/// The frame of the animation that the player's avatar is currently on.
#[derive(Debug, Default, Resource, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct AnimationFrame(pub usize);

impl AnimationFrame {
    /// The maximum number of frames in the animation.
    pub const MAX: usize = 3;

    /// Resets the frame to 0.
    pub fn reset(&mut self) {
        self.0 = 0;
    }
    /// Advances the frame by 1, wrapping around to 0 after hitting `MAX`.
    pub fn next(&mut self) {
        self.0 = (self.0 + 1) % Self::MAX;
    }
    /// Give the frame in the context of an animation's length.
    #[must_use]
    pub const fn get(self, length: usize) -> usize {
        self.0 % length
    }
    /// Returns true if the frame is at the end of the animation.
    #[must_use]
    pub const fn is_fin(self) -> bool {
        self.0 == Self::MAX - 1
    }
}

/// Set the casting animation when spell casting event is triggered.
pub(super) fn set_casting_animation(
    mut supplemental_state_next: ResMut<NextState<PlayerAnimationSupplemental>>,
    mut er_cast_spell: EventReader<CastSpell>,
    mut frame: ResMut<AnimationFrame>,
) {
    // If the player is casting a spell, set the supplemental state to casting.
    if er_cast_spell.read().next().is_some() {
        supplemental_state_next.set(PlayerAnimationSupplemental::Casting);
        frame.reset();
    }
}

/// Advance the animation frame every 0.1 seconds.
pub(super) fn advance_animation_timer(
    time: Res<Time>,
    mut timer: ResMut<PlayerAnimationTimer>,
    mut frame: ResMut<AnimationFrame>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    frame.next();
}

/// The system that updates the avatar's animation based on its state.
///
/// Tracks a local "frame" that goes from 0 to 3, and updates the sprite index based on the current
/// state. We advance the frame every 0.1 seconds.
pub fn update_avatar_animation(
    mut sprite_query: Query<&mut TextureAtlasSprite, With<PlayerAvatar>>,
    state: Res<State<PlayerAnimation>>,
    supplemental_state: Res<State<PlayerAnimationSupplemental>>,
    facing: Res<State<PlayerFacing>>,
    mut supplemental_state_next: ResMut<NextState<PlayerAnimationSupplemental>>,
    frame: Res<AnimationFrame>,
) {
    // should be just one player avatar & sprite
    let Ok(mut sprite) = sprite_query.get_single_mut() else {
        tracing::error!("update_avatar_animation: failed to get player sprite");
        return;
    };

    // Flip the sprite based on the facing state
    sprite.flip_x = *facing.get() == PlayerFacing::Left;

    match (state.get(), supplemental_state.get()) {
        (PlayerAnimation::Walking, PlayerAnimationSupplemental::None) => {
            sprite.index = WALKING_ANIMATION[frame.get(WALKING_ANIMATION.len())];
        }
        (PlayerAnimation::Walking, PlayerAnimationSupplemental::Casting) => {
            sprite.index = CASTING_WALKING_ANIMATION[frame.get(CASTING_WALKING_ANIMATION.len())];
        }
        (PlayerAnimation::Idle, PlayerAnimationSupplemental::None) => {
            sprite.index = IDLE_ANIMATION[frame.get(IDLE_ANIMATION.len())];
        }
        (PlayerAnimation::Idle, PlayerAnimationSupplemental::Casting) => {
            sprite.index = CASTING_IDLE_ANIMATION[frame.get(CASTING_IDLE_ANIMATION.len())];
        }
    }

    // If we're casting, reset the supplemental state after the casting animation is done
    if frame.is_fin() && supplemental_state.get() == &PlayerAnimationSupplemental::Casting {
        supplemental_state_next.set(PlayerAnimationSupplemental::None);
    }
}
