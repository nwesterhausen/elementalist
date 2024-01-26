use bevy::{app::AppExit, prelude::*};

/// System to cleanup entities and then exit the app.
///
/// This should be setup for [`OnEnter(AppState::Cleanup)`]
pub fn cleanup_then_exit(mut app_exit_events: EventWriter<AppExit>) {
    // Do any entity cleanup and saving here..
    // Then exit the app
    app_exit_events.send(AppExit);
}
