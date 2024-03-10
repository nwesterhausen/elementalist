//! Texture atlas resource for storing the texture atlas handle and the texture handle.
use bevy::prelude::*;

/// `TextureAtlas` has been changed in bevy 0.13 so we need to track the texture handle and the texture atlas.
#[derive(Resource, Debug, Clone, Default)]
#[allow(clippy::module_name_repetitions)]
pub struct StoredTextureAtlas {
    /// The texture handle of the texture atlas layout.
    pub atlas_handle: Handle<TextureAtlasLayout>,
    /// The texture handle used by the atlas.
    pub texture_handle: Handle<Image>,
}
