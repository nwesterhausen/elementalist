//! Settings resources for the game.
//!
//! The base settings menu should have these options:
//!
//! - Accessibility
//! - Audio
//! - Video
//! - Gameplay
//! - Controls
//! - Back/Close
//!
//! Each of these options should have a sub-menu, which can be navigated to by pressing select or
//! clicking on the option. The sub-menu should have a back button, which returns to the main menu.
//!
//! The accessibility menu should have these options:
//!
//! - Font Choice (Default, Dyslexic, Sans-Serif)
//! - Back
//!
//! The audio menu should have these options:
//!
//! - Main Volume
//! - Music Volume
//! - SFX Volume
//! - Back
//!
//! The video menu should have these options:
//!
//! - Display Scale
//! - HUD Scaling
//! - Back
//!
//! The gameplay menu should have these options:
//!
//! - Auto-Aim
//! - Auto-Cast
//! - Back
//!
//! The controls menu should have these options:
//!
//! - Keybinds
//! - Keybinds (Controller)
//! - Back
//!
//! The keybinds menu should have these options (and these are the same for controller):
//!
//! - (Options for each action -- see [`crate::events::PlayerAction`])
//! - (Options for each menu interaction -- see [`crate::events::MenuInteraction`])
//! - Back
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

use crate::{
    font_resource::{FontChoice, FontFamily},
    CameraScaleLevel, Volume,
};

/// Volume settings.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    PartialEq,
    Eq,
    Serialize,
    Resource,
    Reflect,
    InspectorOptions,
)]
#[reflect(InspectorOptions)]
#[allow(clippy::module_name_repetitions)]
pub struct VolumeSettings {
    /// Main volume.
    pub main: Volume,
    /// Music volume.
    pub music: Volume,
    /// SFX volume.
    pub sfx: Volume,
}

/// Video settings.
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Serialize, Resource, Reflect, InspectorOptions,
)]
#[reflect(InspectorOptions)]
#[allow(clippy::module_name_repetitions)]
pub struct VideoSettings {
    /// Display scale.
    pub display_scale: CameraScaleLevel,
    /// HUD scale.
    pub hud_scale: f32,
}

/// Gameplay settings.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    PartialEq,
    Eq,
    Serialize,
    Resource,
    Reflect,
    InspectorOptions,
)]
#[reflect(InspectorOptions)]
#[allow(clippy::module_name_repetitions)]
pub struct GameplaySettings {
    /// Auto-aim.
    pub auto_aim: bool,
    /// Auto-cast.
    pub auto_cast: bool,
}

/// Accessibility settings.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    PartialEq,
    Eq,
    Serialize,
    Resource,
    Reflect,
    InspectorOptions,
)]
#[reflect(InspectorOptions)]
#[allow(clippy::module_name_repetitions)]
pub struct AccessibilitySettings {
    /// Display Font choice (default, dyslexic, sans-serif).
    pub display_font_family: FontFamily,
    /// Interface Font choice (default, dyslexic, sans-serif).
    pub interface_font_family: FontFamily,
    /// Main Font choice (default, dyslexic, sans-serif).
    pub main_font_family: FontFamily,
}

/// Rotates through the font choices.
///
/// If given `FontChoice::All`, it will return `FontChoice::Display` (All is not a valid choice).
///
/// # Example
///
/// ```
/// use game_library::{font_resource::FontChoice, settings::next_font_choice};
///
/// assert_eq!(next_font_choice(FontChoice::Display), FontChoice::Interface);
/// assert_eq!(next_font_choice(FontChoice::Interface), FontChoice::Main);
/// assert_eq!(next_font_choice(FontChoice::Main), FontChoice::Console);
/// assert_eq!(next_font_choice(FontChoice::Console), FontChoice::Display);
/// assert_eq!(next_font_choice(FontChoice::All), FontChoice::Display);
/// ```
#[must_use]
pub const fn next_font_choice(font_choice: FontChoice) -> FontChoice {
    match font_choice {
        FontChoice::Display => FontChoice::Interface,
        FontChoice::Interface => FontChoice::Main,
        FontChoice::Main => FontChoice::Console,
        FontChoice::Console | FontChoice::All => FontChoice::Display,
    }
}

/// Rotates through the font families. This will not return `FontFamily::Display` because
/// that is a special font family used for the game logo.
///
/// # Example
///
/// ```
/// use game_library::{font_resource::FontFamily, settings::next_font_family};
///
/// assert_eq!(next_font_family(FontFamily::Display), FontFamily::Dyslexic);
/// assert_eq!(next_font_family(FontFamily::Fancy), FontFamily::Dyslexic);
/// assert_eq!(next_font_family(FontFamily::Dyslexic), FontFamily::SansSerif);
/// assert_eq!(next_font_family(FontFamily::SansSerif), FontFamily::Monospace);
/// assert_eq!(next_font_family(FontFamily::Monospace), FontFamily::Fancy);
/// ```
#[must_use]
pub const fn next_font_family(font_family: FontFamily) -> FontFamily {
    match font_family {
        FontFamily::Display | FontFamily::Fancy => FontFamily::Dyslexic,
        FontFamily::Dyslexic => FontFamily::SansSerif,
        FontFamily::SansSerif => FontFamily::Monospace,
        FontFamily::Monospace => FontFamily::Fancy,
    }
}

/// Plugin for settings which will register the settings resources and run the `first_load` system.
///
/// This also registers the [`SettingChanged`] event and a system to flush the settings to the
/// [`bevy_pkv::PkvStore`] when the [`SettingChanged`] event is sent.
#[allow(clippy::module_name_repetitions)]
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SettingChanged>()
            .init_resource::<VolumeSettings>()
            .init_resource::<VideoSettings>()
            .init_resource::<GameplaySettings>()
            .init_resource::<AccessibilitySettings>()
            .add_systems(Startup, first_load)
            .add_systems(Update, flush_settings_to_store);
    }
}

/// Event to indicate a setting was changed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Event)]
pub struct SettingChanged(pub SettingCategory);

/// The category of a setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum SettingCategory {
    /// Volume settings.
    Volume,
    /// Video settings.
    Video,
    /// Gameplay settings.
    Gameplay,
    /// Accessibility settings.
    Accessibility,
}

impl SettingCategory {
    /// Returns the name of the setting category.
    ///
    /// This could be used as a key to save in the [`bevy_pkv::PkvStore`].
    ///
    /// # Example
    ///
    /// ```
    /// use game_library::settings::SettingCategory;
    ///
    /// assert_eq!(SettingCategory::Volume.name(), "Volume");
    /// ```
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Volume => "Volume",
            Self::Video => "Video",
            Self::Gameplay => "Gameplay",
            Self::Accessibility => "Accessibility",
        }
    }
}

/// System to run on the first load of the game. It will either load the settings from the
/// [`bevy_pkv::PkvStore`] or store default settings in the [`bevy_pkv::PkvStore`].
fn first_load(
    mut volume_settings: ResMut<VolumeSettings>,
    mut video_settings: ResMut<VideoSettings>,
    mut gameplay_settings: ResMut<GameplaySettings>,
    mut accessibility_settings: ResMut<AccessibilitySettings>,
    mut pkv_store: ResMut<PkvStore>,
) {
    // Load the settings from the pkv store.
    if let Ok(volume) = pkv_store.get::<VolumeSettings>(SettingCategory::Volume.name()) {
        *volume_settings = volume;
    } else {
        let _ = pkv_store
            .set(SettingCategory::Volume.name(), &VolumeSettings::default())
            .map_err(|err| {
                tracing::error!("failed to save volume settings to disk: {}", err);
            });
    }

    if let Ok(video) = pkv_store.get::<VideoSettings>(SettingCategory::Video.name()) {
        *video_settings = video;
    } else {
        let _ = pkv_store
            .set(SettingCategory::Video.name(), &VideoSettings::default())
            .map_err(|err| {
                tracing::error!("failed to save video settings to disk: {}", err);
            });
    }

    if let Ok(gameplay) = pkv_store.get::<GameplaySettings>(SettingCategory::Gameplay.name()) {
        *gameplay_settings = gameplay;
    } else {
        let _ = pkv_store
            .set(
                SettingCategory::Gameplay.name(),
                &GameplaySettings::default(),
            )
            .map_err(|err| {
                tracing::error!("failed to save gameplay settings to disk: {}", err);
            });
    }

    if let Ok(accessibility) =
        pkv_store.get::<AccessibilitySettings>(SettingCategory::Accessibility.name())
    {
        *accessibility_settings = accessibility;
    } else {
        let _ = pkv_store
            .set(
                SettingCategory::Accessibility.name(),
                &AccessibilitySettings::default(),
            )
            .map_err(|err| {
                tracing::error!("failed to save accessibility settings to disk: {}", err);
            });
    }
}

/// System that runs on [`Update`] and reacts to the [`SettingChanged`] event.
///
/// This will save the settings to the [`bevy_pkv::PkvStore`].
#[allow(clippy::needless_pass_by_value)]
fn flush_settings_to_store(
    volume_settings: Res<VolumeSettings>,
    video_settings: Res<VideoSettings>,
    gameplay_settings: Res<GameplaySettings>,
    accessibility_settings: Res<AccessibilitySettings>,
    mut pkv_store: ResMut<PkvStore>,
    mut setting_changed_events: EventReader<SettingChanged>,
) {
    for setting_changed_event in setting_changed_events.read() {
        match setting_changed_event.0 {
            SettingCategory::Volume => {
                let _ = pkv_store
                    .set(SettingCategory::Volume.name(), &*volume_settings)
                    .map_err(|err| {
                        tracing::error!("failed to save volume settings to disk: {}", err);
                    });
            }
            SettingCategory::Video => {
                let _ = pkv_store
                    .set(SettingCategory::Video.name(), &*video_settings)
                    .map_err(|err| {
                        tracing::error!("failed to save video settings to disk: {}", err);
                    });
            }
            SettingCategory::Gameplay => {
                let _ = pkv_store
                    .set(SettingCategory::Gameplay.name(), &*gameplay_settings)
                    .map_err(|err| {
                        tracing::error!("failed to save gameplay settings to disk: {}", err);
                    });
            }
            SettingCategory::Accessibility => {
                let _ = pkv_store
                    .set(
                        SettingCategory::Accessibility.name(),
                        &*accessibility_settings,
                    )
                    .map_err(|err| {
                        tracing::error!("failed to save accessibility settings to disk: {}", err);
                    });
            }
        }
    }
}
