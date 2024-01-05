use bevy::{app::AppExit, prelude::*};

use super::buttons;
use super::buttons::ButtonAction;
use super::scenes;
use super::tags;
use super::MenuState;
use crate::despawn_screen;
use crate::settings::AudioSettings;
use crate::settings::VideoSettings;
use crate::GameState;

/// The main menu plugin. This plugin is responsible for setting up the main menu.
///
/// It includes the following screens:
///
/// * Main menu (continue, new game, settings, quit)
/// * Settings (audio, video, controls, gameplay, back)
/// * Audio settings
/// * Video settings
/// * Controls settings
/// * Gameplay settings
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // We manage a "menu state" for the various screens
            .add_state::<MenuState>()
            // Setup the main menu when entered
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            // Handle the main menu state
            .add_systems(OnEnter(MenuState::Main), scenes::main_menu_setup)
            .add_systems(
                OnExit(MenuState::Main),
                despawn_screen::<tags::OnMainMenuScreen>,
            )
            // Handle the settings screen
            .add_systems(OnEnter(MenuState::Settings), scenes::settings_menu_setup)
            .add_systems(
                OnExit(MenuState::Settings),
                despawn_screen::<tags::OnSettingsMenuScreen>,
            )
            // Handle the display settings screen
            .add_systems(
                OnEnter(MenuState::SettingsDisplay),
                scenes::display_settings_setup,
            )
            .add_systems(
                Update,
                (buttons::setting_button::<VideoSettings::DisplayQuality>
                    .run_if(in_state(MenuState::SettingsDisplay))),
            )
            .add_systems(
                OnExit(MenuState::SettingsDisplay),
                despawn_screen::<tags::OnDisplaySettingsMenuScreen>,
            )
            // Handle the sound settings screen
            .add_systems(
                OnEnter(MenuState::SettingsSound),
                scenes::sound_settings_setup,
            )
            .add_systems(
                Update,
                (buttons::setting_button::<AudioSettings::Volume>
                    .run_if(in_state(MenuState::SettingsSound))),
            )
            .add_systems(
                OnExit(MenuState::SettingsSound),
                despawn_screen::<tags::OnSoundSettingsMenuScreen>,
            )
            // Handle the controls settings screen
            .add_systems(
                OnEnter(MenuState::SettingsControls),
                scenes::controls_settings_setup,
            )
            .add_systems(
                OnExit(MenuState::SettingsControls),
                despawn_screen::<tags::OnControlsSettingsMenuScreen>,
            )
            // Handle the gameplay settings screen
            .add_systems(
                OnEnter(MenuState::SettingsGameplay),
                scenes::gameplay_settings_setup,
            )
            .add_systems(
                OnExit(MenuState::SettingsGameplay),
                despawn_screen::<tags::OnGameplaySettingsMenuScreen>,
            )
            // Common systems to all screens that handles buttons behavior
            .add_systems(
                Update,
                (menu_action, buttons::button_system).run_if(in_state(GameState::MainMenu)),
            );
    }
}

/// System to handle the main menu button actions
fn menu_action(
    interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                ButtonAction::Quit => app_exit_events.send(AppExit),
                ButtonAction::Play => {
                    game_state.set(GameState::Game);
                    menu_state.set(MenuState::Disabled);
                }
                ButtonAction::Settings => menu_state.set(MenuState::Settings),
                ButtonAction::DisplaySettings => {
                    menu_state.set(MenuState::SettingsDisplay);
                }
                ButtonAction::SoundSettings => {
                    menu_state.set(MenuState::SettingsSound);
                }
                ButtonAction::ControlsSettings => {
                    menu_state.set(MenuState::SettingsControls);
                }
                ButtonAction::GameplaySettings => {
                    menu_state.set(MenuState::SettingsGameplay);
                }
                ButtonAction::BackToMain => {
                    menu_state.set(MenuState::Main);
                }
                ButtonAction::BackToSettings => {
                    menu_state.set(MenuState::Settings);
                }
                _ => tracing::error!("Unhandled button action: {:?}", menu_button_action),
            }
        }
    }
}

/// System to setup the main menu
///
/// When the main menu is entered, we setup the main menu state to the main menu.
fn setup_main_menu(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}
