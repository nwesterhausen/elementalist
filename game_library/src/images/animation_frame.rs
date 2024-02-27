//! Component for tracking the current frame of an animation.
use bevy::prelude::*;

/// The frame of the animation that the sprite is currently on.
///
/// This is used to track the current frame of an animation, and to advance the frame.
#[derive(Debug, Resource, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct AnimationFrame {
    /// The current frame of the animation.
    pub frame: usize,
    /// The maximum number of frames in the animation.
    max: usize,
}

impl AnimationFrame {
    /// Create a new animation frame tracker.
    #[must_use]
    pub const fn new(max: usize) -> Self {
        Self { frame: 0, max }
    }

    /// Update the maximum number of frames in the animation.
    ///
    /// This will reset the current frame to 0.
    pub fn update_max(&mut self, max: usize) {
        self.max = max;
        self.reset();
    }

    /// Get the maximum number of frames in the animation.
    #[must_use]
    pub const fn max(self) -> usize {
        self.max
    }

    /// Resets the frame to 0.
    pub fn reset(&mut self) {
        self.frame = 0;
    }
    /// Advances the frame by 1, wrapping around to 0 after hitting `MAX`.
    pub fn next(&mut self) {
        self.frame = (self.frame + 1) % self.max;
    }
    /// Give the frame in the context of an animation's length.
    #[must_use]
    pub const fn get(self, length: usize) -> usize {
        self.frame % length
    }
    /// Returns true if the frame is at the end of the animation.
    #[must_use]
    pub const fn is_fin(self) -> bool {
        self.frame == self.max - 1
    }
}
