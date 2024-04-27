//! Timer to control when animations should advance.
use bevy::prelude::*;

/// Timer that will be used to update the animation. This is specialized for animations.
///
/// Set the duration and update the timer with the `advance` method.
///
/// The duration can be changed with the `update_duration` method. This will reset the timer.
#[derive(Debug, Default, Resource, Clone, Component)]
pub struct AnimationTimer {
    /// The timer.
    timer: Timer,
    /// The duration of the timer.
    duration: f32,
}

impl AnimationTimer {
    /// Create a new animation timer.
    ///
    /// ## Parameters
    ///
    /// - `duration`: The duration of the timer in seconds.
    #[must_use]
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Repeating),
            duration,
        }
    }

    /// Get the duration of the timer.
    #[must_use]
    pub const fn duration(&self) -> f32 {
        self.duration
    }

    /// Update the animation state and timer duration.
    ///
    /// This will reset the timer.
    ///
    /// ## Parameters
    ///
    /// - `duration`: The new duration of the timer in seconds.
    pub fn update(&mut self, duration: f32) {
        self.duration = duration;
        self.timer = Timer::from_seconds(duration, TimerMode::Repeating);
    }

    /// Advance the timer.
    ///
    /// Returns true if the timer has just finished.
    ///
    /// # Parameters
    ///
    /// - `time`: The time resource to use to advance the timer. (i.e. `Res<Time>`)
    pub fn advance(&mut self, time: &Time) -> bool {
        self.timer.tick(time.delta()).just_finished()
    }

    /// Check if timer is finished.
    #[must_use]
    pub fn finished(&self) -> bool {
        self.timer.finished()
    }

    /// Pause the timer.
    pub fn pause(&mut self) {
        self.timer.pause();
    }

    /// Resume the timer.
    pub fn resume(&mut self) {
        self.timer.unpause();
    }
}