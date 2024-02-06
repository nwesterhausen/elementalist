use bevy::prelude::*;

use crate::font_resource::FontResource;

use super::resources::GenerationSeed;

/// Draws the seed on the screen.
#[allow(clippy::needless_pass_by_value)]
pub(super) fn draw_seed_on_screen(
    fonts: Res<FontResource>,
    seed: Res<GenerationSeed>,
    mut commands: Commands,
) {
    // Write the seed in the top right corner of the screen
    let seed_text = format!("Seed: {}", seed.0);
    let font = fonts.console_font.clone();

    commands.spawn(TextBundle {
        text: Text::from_section(
            seed_text,
            TextStyle {
                font,
                font_size: 12.0,
                color: Color::rgba(1.0, 1.0, 1.0, 0.25),
            },
        ),
        style: Style {
            position_type: PositionType::Absolute,
            top: Val::Px(16.0),
            right: Val::Px(8.0),
            ..default()
        },
        ..default()
    });
}
