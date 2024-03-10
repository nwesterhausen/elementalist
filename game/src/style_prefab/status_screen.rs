use bevy::prelude::*;
use game_library::colors;

/// The stats node style for the status screen. We want the stats to be in a column down the left
/// side of the screen.
#[must_use]
pub fn status_screen_stats_node_bundle() -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Start,
            width: Val::Percent(50.0),
            margin: UiRect::px(20., 0., 40., 0.),
            ..default()
        },
        ..default()
    }
}

/// Status screen shows the player's stats. The text is the stat name and value, and we need ot make
/// it small enough to fit a lot of stats on the screen.
#[must_use]
pub fn status_screen_text_bundle(text: impl Into<String>, font: Handle<Font>) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font_size: 16.0,
            color: colors::TEXT_COLOR,
            font,
        },
    )
}
