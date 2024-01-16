use bevy::prelude::*;

use super::{components::*, resources::SplashTimer};

/// System that draws the splash screen
pub fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("sprite/wizard.png");
    // Display the logo
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    // This will set the logo to be 32px wide, and auto adjust its height
                    width: Val::Px(32.0),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Elementalist",
                    TextStyle {
                        font: asset_server.load("ui/fonts/Almendra-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                ),
                style: Style {
                    margin: UiRect::all(Val::Px(60.0)),
                    ..default()
                },
                ..default()
            });
        });
    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}
