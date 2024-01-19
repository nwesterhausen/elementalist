use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Reflect, InspectorOptions, Clone, Copy)]
#[reflect(Component, InspectorOptions)]
pub struct CameraScaleLevel {
    value: f32,
    index: usize,
}

impl Default for CameraScaleLevel {
    fn default() -> Self {
        Self {
            value: 0.3,
            index: 3,
        }
    }
}

impl CameraScaleLevel {
    pub const LEVELS: [f32; 5] = [0.1, 0.25, 0.3, 0.5, 1.0];

    pub fn increase(&mut self) {
        self.index += 1;

        if self.index >= Self::LEVELS.len() {
            self.index = Self::LEVELS.len() - 1;
        }

        self.value = Self::LEVELS[self.index];
    }

    pub fn decrease(&mut self) {
        self.index -= 1;

        if self.index >= Self::LEVELS.len() {
            self.index = 0;
        }

        self.value = Self::LEVELS[self.index];
    }
}

/// Spawns a basic camera
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera,
        CameraScaleLevel::default(),
    ));
}

/// Zooms the camera based on the current scale level
pub fn zoom_camera(
    mut query: Query<(&mut OrthographicProjection, &CameraScaleLevel), With<MainCamera>>,
) {
    for (mut projection, scale_level) in &mut query {
        projection.scale = scale_level.value;
    }
}
