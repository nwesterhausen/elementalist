use super::{despawn_screen, GameState};
use bevy::prelude::*;

/// This shows a splash screen with something while it loads
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // Focused on the splash screen game state
        app
            // When entering the splash screen, load/spawn the splash screen
            .add_systems(OnEnter(GameState::Splash), splash_setup)
            // While in the splash screen, countdown until the splash screen is done
            .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
            // When exiting the splash screen, despawn the splash screen
            .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
    }
}

/// Tag which identifies the splash screen and its entities
#[derive(Component)]
struct OnSplashScreen;

/// The splash screen countdown timer
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("panda.png");
    // Display the logo/icon
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                    ..Default::default()
                },
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    // Sets the logo icon to be 200px
                    width: Val::Px(200.0),
                    ..Default::default()
                },
                image: UiImage::new(icon),
                ..Default::default()
            });
        });
    // Setup the countdown timer
    commands.insert_resource(SplashTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut splash_timer: ResMut<SplashTimer>,
) {
    if splash_timer.tick(time.delta()).finished() {
        game_state.set(GameState::MainMenu);
    }
}
