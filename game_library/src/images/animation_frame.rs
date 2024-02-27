//! Component for tracking the current frame of an animation.
use bevy::prelude::*;

const MAX_SUPPORTED_ANIMATION_FRAMES: usize = 30;

/// The frame of the animation that the sprite is currently on.
///
/// This is used to track the current frame of an animation, and to advance the frame.
#[derive(Debug, Resource, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct AnimationFrame {
    /// The current frame of the animation.
    pub frame: usize,
}

impl AnimationFrame {
    /// The maximum number of frames supported by the animation frame tracker.
    pub const MAX: usize = MAX_SUPPORTED_ANIMATION_FRAMES;

    /// Create a new animation frame tracker.
    #[must_use]
    pub const fn new() -> Self {
        Self { frame: 0 }
    }

    /// Resets the frame to 0.
    pub fn reset(&mut self) {
        self.frame = 0;
    }
    /// Advances the frame by 1, wrapping around to 0 after hitting `Self::MAX`.
    pub fn next(&mut self) {
        self.frame = (self.frame + 1) % Self::MAX;
    }
    /// Give the frame in the context of an animation's length.
    ///
    /// This is a safe way to get a frame that is within the bounds of an animation's length.
    #[must_use]
    pub const fn get(self, length: usize) -> usize {
        self.frame % length
    }
    /// Returns true if the frame is at the end of the animation.
    #[must_use]
    pub const fn is_fin(self, length: usize) -> bool {
        self.frame >= length
    }
}
