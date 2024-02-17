//! Helps with z-index for 2d sprites
use bevy::prelude::*;

use crate::GeneratedMaps;

/// This plugin is used to update the z coordinate of the transform based on the layer
pub struct LayerPlugin;

impl Plugin for LayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (update_layer_depth, update_z_coordinate_based_on_layer).chain(),
        );
    }
}

/// The layer of the sprite. The lower the value, the further back the sprite is.
///
/// Each layer is meant to be subdivided into parts. This is to allow for fine grained control over the z-index.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
pub enum Layer {
    /// The background layer (ground, etc.) What should be behind everything
    Background(i16),
    /// The foreground layer (player, enemies, etc.) What should be in front of everything
    Foreground(i16),
}

const LAYER_MODIFIER: f32 = 1000.;

fn update_z_coordinate_based_on_layer(mut query: Query<(&mut Transform, &Layer), Changed<Layer>>) {
    for (mut transform, layer) in &mut query {
        transform.translation.z = match layer {
            Layer::Background(order_in_layer) => -5. + f32::from(*order_in_layer) / LAYER_MODIFIER,
            Layer::Foreground(order_in_layer) => 5. + f32::from(*order_in_layer) / LAYER_MODIFIER,
        }
    }
}

/// System to update the Layer depth based on translation.y value
#[allow(clippy::needless_pass_by_value)]
fn update_layer_depth(
    mut query: Query<(&mut Layer, &Transform), Changed<Transform>>,
    generated_map: Res<GeneratedMaps>,
) {
    for (mut layer, transform) in &mut query {
        // create depth from f32 to i16 with the f32 range mapping to i16 range, since translation.y has valid
        // values between -0.5 * generated_map.dimensions().1 and 0.5 * generated_map.dimensions().1
        let depth = (transform.translation.y / (0.5 * generated_map.dimensions().1 as f32)
            * f32::from(i16::MAX)) as i16;

        *layer = match *layer {
            Layer::Background(_) => Layer::Background(depth),
            Layer::Foreground(_) => Layer::Foreground(depth),
        };
    }
}
