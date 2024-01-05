use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct VideoSettingsBundle {
    pub display_quality: DisplayQuality,
}

#[derive(
    Resource, Default, Debug, Component, PartialEq, Eq, Clone, Copy, Serialize, Deserialize,
)]
pub enum DisplayQuality {
    Low,
    #[default]
    Medium,
    High,
}
