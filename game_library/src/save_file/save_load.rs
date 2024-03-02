//! Module for loading save files.

use super::{
    data::SaveFile,
    events::{Action, SaveFileEvent},
    paths::SaveFileDirectories,
};
use bevy::prelude::*;

/// System to load a save file.
#[allow(clippy::needless_pass_by_value)]
pub(super) fn handle_save_file_event(
    mut commands: Commands,
    save_file: Res<SaveFile>,
    save_file_directories: Res<SaveFileDirectories>,
    mut event_ro: EventReader<SaveFileEvent>,
) {
    for event in event_ro.read() {
        match event.action {
            Action::Load => {
                let Some(save_file) = SaveFile::load(
                    &save_file_directories
                        .save
                        .join(&format!("{}.sav", event.name)),
                ) else {
                    error!("Unable to load save file: {}", event.name);
                    continue;
                };
                commands.insert_resource(save_file);
            }
            Action::Save | Action::Update => {
                save_file.save(&save_file_directories.save);
            }
        }
    }
}
