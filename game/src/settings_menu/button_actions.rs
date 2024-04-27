//! Actions that can be performed by the buttons in the settings menu.

use bevy::prelude::*;

use game_library::{
    buttons::ButtonSystem,
    state::{AppState, Overlay, Settings},
};

use super::events::{ChangeSetting, IndividualSetting};

/// Resource to hold all the button systems for the settings menu.
#[derive(Resource)]
pub(super) struct SettingsButtons {
    /// Button to close the menu
    pub close: ButtonSystem,
    /// Button to go back to the main settings menu
    pub back_to_menu: ButtonSystem,
    /// Button to go to the audio settings
    pub show_audio_settings: ButtonSystem,
    /// Button to go to the display settings
    pub show_video_settings: ButtonSystem,
    /// Button to go to the controls settings
    pub show_controls_settings: ButtonSystem,
    /// Button to go to the gameplay settings
    pub show_gameplay_settings: ButtonSystem,
    /// Button to go to the accessibility settings
    pub show_accessibility_settings: ButtonSystem,
    /// Button to toggle auto-cast
    pub toggle_auto_cast: ButtonSystem,
    /// Button to toggle auto-aim
    pub toggle_auto_aim: ButtonSystem,
    /// Button to change font family
    pub rotate_font_family: ButtonSystem,
    /// Button to increment the main volume
    /// increases the volume of all sounds and music played in the game.
    pub increment_main_volume: ButtonSystem,
    /// Button to increment music volume
    pub increment_music_volume: ButtonSystem,
    /// Button to increment sound effects volume
    pub increment_sound_effects: ButtonSystem,
    /// Button to go back to the main menu
    pub quit_to_main_menu: ButtonSystem,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct SettingsMenuButton;

/// System for handling "settings menu close" action
pub(super) fn button_close_settings(
    mut menu_state: ResMut<NextState<Settings>>,
    mut overlay_state: ResMut<NextState<Overlay>>,
) {
    menu_state.set(Settings::Disabled);
    overlay_state.set(Overlay::None);
}

/// System for handling "back to main settings menu" action
pub(super) fn button_back_to_menu(mut menu_state: ResMut<NextState<Settings>>) {
    menu_state.set(Settings::Main);
}

/// System to handle "go to audio settings" action
pub(super) fn button_go_to_audio_settings(mut menu_state: ResMut<NextState<Settings>>) {
    menu_state.set(Settings::Audio);
}

/// System to handle "go to display settings" action
pub(super) fn button_go_to_display_settings(mut menu_state: ResMut<NextState<Settings>>) {
    menu_state.set(Settings::Display);
}

/// System to handle "go to controls settings" action
pub(super) fn button_go_to_controls_settings(mut menu_state: ResMut<NextState<Settings>>) {
    menu_state.set(Settings::Controls);
}

/// System to handle "go to gameplay settings" action
pub(super) fn button_go_to_gameplay_settings(mut menu_state: ResMut<NextState<Settings>>) {
    menu_state.set(Settings::Gameplay);
}

/// System to handle "go to accessibility settings" action
pub(super) fn button_go_to_accessibility_settings(mut menu_state: ResMut<NextState<Settings>>) {
    menu_state.set(Settings::Accessibility);
}

/// System to handle "go to main menu" action
pub(super) fn button_go_to_main(
    mut menu_state: ResMut<NextState<Settings>>,
    mut game_state: ResMut<NextState<AppState>>,
) {
    menu_state.set(Settings::Disabled);
    game_state.set(AppState::MainMenu);
}

/// System to handle "toggle auto cast" action
pub(super) fn button_toggle_auto_cast(mut ew_change_setting: EventWriter<ChangeSetting>) {
    ew_change_setting.send(ChangeSetting {
        setting: IndividualSetting::AutoCast,
    });
}

/// System to handle "toggle auto aim" action
pub(super) fn button_toggle_auto_aim(mut ew_change_setting: EventWriter<ChangeSetting>) {
    ew_change_setting.send(ChangeSetting {
        setting: IndividualSetting::AutoAim,
    });
}

/// System to handle "rotate font family" action
pub(super) fn button_rotate_font_family(mut ew_change_setting: EventWriter<ChangeSetting>) {
    ew_change_setting.send(ChangeSetting {
        setting: IndividualSetting::FontFamily,
    });
}

/// System to handle "increment main volume" action
pub(super) fn button_increment_main_volume(mut ew_change_setting: EventWriter<ChangeSetting>) {
    ew_change_setting.send(ChangeSetting {
        setting: IndividualSetting::MainVolume,
    });
}

/// System to handle "increment music volume" action
pub(super) fn button_increment_music_volume(mut ew_change_setting: EventWriter<ChangeSetting>) {
    ew_change_setting.send(ChangeSetting {
        setting: IndividualSetting::MusicVolume,
    });
}

/// System to handle "increment sound effects volume" action
pub(super) fn button_increment_sound_effects_volume(
    mut ew_change_setting: EventWriter<ChangeSetting>,
) {
    ew_change_setting.send(ChangeSetting {
        setting: IndividualSetting::SoundEffectsVolume,
    });
}

/// System to create the button resources for the settings menu.
pub(super) fn initialize_settings_buttons(world: &mut World) {
    let close = ButtonSystem {
        pressed_event: Some(world.register_system(button_close_settings)),
    };
    let back_to_menu = ButtonSystem {
        pressed_event: Some(world.register_system(button_back_to_menu)),
    };
    let show_audio_settings = ButtonSystem {
        pressed_event: Some(world.register_system(button_go_to_audio_settings)),
    };
    let show_video_settings = ButtonSystem {
        pressed_event: Some(world.register_system(button_go_to_display_settings)),
    };
    let show_controls_settings = ButtonSystem {
        pressed_event: Some(world.register_system(button_go_to_controls_settings)),
    };
    let show_gameplay_settings = ButtonSystem {
        pressed_event: Some(world.register_system(button_go_to_gameplay_settings)),
    };
    let show_accessibility_settings = ButtonSystem {
        pressed_event: Some(world.register_system(button_go_to_accessibility_settings)),
    };
    let toggle_auto_cast = ButtonSystem {
        pressed_event: Some(world.register_system(button_toggle_auto_cast)),
    };
    let toggle_auto_aim = ButtonSystem {
        pressed_event: Some(world.register_system(button_toggle_auto_aim)),
    };
    let rotate_font_family = ButtonSystem {
        pressed_event: Some(world.register_system(button_rotate_font_family)),
    };
    let increment_main_volume = ButtonSystem {
        pressed_event: Some(world.register_system(button_increment_main_volume)),
    };
    let increment_music_volume = ButtonSystem {
        pressed_event: Some(world.register_system(button_increment_music_volume)),
    };
    let increment_sound_effects = ButtonSystem {
        pressed_event: Some(world.register_system(button_increment_sound_effects_volume)),
    };
    let quit_to_main_menu = ButtonSystem {
        pressed_event: Some(world.register_system(button_go_to_main)),
    };

    world.insert_resource(SettingsButtons {
        close,
        back_to_menu,
        show_audio_settings,
        show_video_settings,
        show_controls_settings,
        show_gameplay_settings,
        show_accessibility_settings,
        toggle_auto_cast,
        toggle_auto_aim,
        rotate_font_family,
        increment_main_volume,
        increment_music_volume,
        increment_sound_effects,
        quit_to_main_menu,
    });
}
