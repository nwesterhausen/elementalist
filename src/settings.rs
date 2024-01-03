use bevy::prelude::*;

/// This is our settings plugin which loads settings as resources and handles them.
pub struct SettingsPlugin;

/// A sample setting that we have in our menu. It will be a resource that we can access from anywhere.
#[derive(Clone, Copy, Default, Eq, PartialEq, Hash, Debug, Resource)]
enum DisplayQuality {
    /// Low quality
    Low,
    #[default]
    /// Medium quality
    Medium,
    /// High quality
    High,
}

/// Another sample setting that we have in our menu. It will be a resource that we can access from anywhere.
///
/// This is a volume setting, which is a `u32` between 0 and 100 (or expected to be).
#[derive(Clone, Copy, Default, Eq, PartialEq, Hash, Debug, Resource)]
struct Volume(u32);

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add resources set to the default values
            .insert_resource(DisplayQuality::Medium)
            .insert_resource(Volume(50));
    }
}
