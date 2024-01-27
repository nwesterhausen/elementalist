use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::bevy_egui::{EguiContext, EguiPlugin};
use bevy_inspector_egui::egui::{self, Align2};

use game_library::settings::{
    AccessibilitySettings, GameplaySettings, VideoSettings, VolumeSettings,
};
use game_library::{
    enums::Skill,
    font_resource::{FontChoice, FontResource},
    Acceleration, Attribute, CameraScaleLevel, CursorPosition, Health, Mana, MovementBundle,
    SpellChoices, Stat, StatBonus, Velocity, Volume,
};

pub struct DevSystemsPlugin;

impl Plugin for DevSystemsPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app.add_plugins(EguiPlugin)
            .register_type::<Attribute>()
            .register_type::<Skill>()
            .register_type::<Stat>()
            .register_type::<StatBonus>()
            .register_type::<Volume>()
            .register_type::<Mana>()
            .register_type::<Health>()
            .register_type::<Velocity>()
            .register_type::<Acceleration>()
            .register_type::<SpellChoices>()
            .register_type::<CursorPosition>()
            .register_type::<CameraScaleLevel>()
            .register_type::<MovementBundle>()
            .register_type::<FontResource>()
            .register_type::<FontChoice>()
            .register_type::<VolumeSettings>()
            .register_type::<VideoSettings>()
            .register_type::<GameplaySettings>()
            .register_type::<AccessibilitySettings>()
            .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin)
            .add_systems(Update, inspector_ui);
    }
}

fn inspector_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        tracing::error!("Failed to get egui context for inspector ui");
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("Inspector")
        .anchor(Align2::RIGHT_TOP, (0., 0.))
        .default_open(false)
        .show(egui_context.get_mut(), |ui| {
            // The world inspector plugin
            bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);

            // Add a collapsable section for the SpellChoices resource.
            egui::CollapsingHeader::new("Spell Choices")
                .default_open(false)
                .show(ui, |ui| {
                    // The resource inspector plugin
                    bevy_inspector_egui::bevy_inspector::ui_for_resource::<SpellChoices>(world, ui);
                });
            // Add collapsable sections for the settings resources.
            egui::CollapsingHeader::new("Volume Settings")
                .default_open(false)
                .show(ui, |ui| {
                    bevy_inspector_egui::bevy_inspector::ui_for_resource::<VolumeSettings>(
                        world, ui,
                    );
                });
            // Add collapsable sections for the settings resources.
            egui::CollapsingHeader::new("Video Settings")
                .default_open(false)
                .show(ui, |ui| {
                    bevy_inspector_egui::bevy_inspector::ui_for_resource::<VideoSettings>(
                        world, ui,
                    );
                });
            // Add collapsable sections for the settings resources.
            egui::CollapsingHeader::new("Gameplay Settings")
                .default_open(false)
                .show(ui, |ui| {
                    bevy_inspector_egui::bevy_inspector::ui_for_resource::<GameplaySettings>(
                        world, ui,
                    );
                });
            // Add collapsable sections for the settings resources.
            egui::CollapsingHeader::new("Accessibility Settings")
                .default_open(false)
                .show(ui, |ui| {
                    bevy_inspector_egui::bevy_inspector::ui_for_resource::<AccessibilitySettings>(
                        world, ui,
                    );
                });
        });
}
