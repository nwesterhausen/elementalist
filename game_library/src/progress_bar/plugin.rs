//! Plugin for adding a progress bar systems to the game.

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::progress_bar::{Percentage, ProgressBarConfig};

use super::BarState;

/// Plugin for adding a progress bar systems to the game.
///
/// This plugin adds a system to draw progress bars for entities with a `Percentage` component.
///
/// To have the progress bars be drawn, add the `ProgressBarConfig` component to the entity to
/// configure how to draw the progress bar on that entity for the specific `Percentage` component.
#[allow(clippy::module_name_repetitions)]
pub struct ProgressBarPlugin<T: Percentage + Component> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: Percentage + Component> Default for ProgressBarPlugin<T> {
    fn default() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Percentage + Component> Plugin for ProgressBarPlugin<T> {
    fn build(&self, app: &mut App) {
        // systems for:
        // - spawn
        // - update
        // - remove
        app.add_systems(Update, (spawn_progress_bars::<T>, remove::<T>, update::<T>));
    }
}

/// Component to store the progress bar entities.
///
/// This component is added to the entity with the `Percentage` component.
///
/// The tuple contains the background and foreground entities (in that order).
#[derive(Component, Reflect)]
struct WithProgressBar<T: Percentage + Component> {
    /// The background bar entity
    pub foreground: Entity,
    /// The foreground bar entity
    pub background: Entity,
    /// Handle for the foreground bar material in the ok state
    pub ok_handle: Handle<ColorMaterial>,
    /// Handle for the foreground bar material in the moderate state
    pub moderate_handle: Handle<ColorMaterial>,
    /// Handle for the foreground bar material in the critical state
    pub critical_handle: Handle<ColorMaterial>,

    /// Marker for the generic type
    #[reflect(ignore)]
    _marker: std::marker::PhantomData<T>,
}

impl<T: Percentage + Component> WithProgressBar<T> {
    const fn get(&self) -> (Entity, Entity) {
        (self.background, self.foreground)
    }

    fn get_material(&self, config: &ProgressBarConfig<T>, percentage: &T) -> Handle<ColorMaterial> {
        match config.get_state(percentage) {
            BarState::Ok => self.ok_handle.clone(),
            BarState::Moderate => self.moderate_handle.clone(),
            BarState::Critical => self.critical_handle.clone(),
        }
    }
}

/// System to spawn progress bars for entities with a `Percentage` component.
///
/// The `Added<T>` query just gets entities which are new and have the `T` component.
#[allow(clippy::needless_pass_by_value)]
fn spawn_progress_bars<T: Percentage + Component>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, &T, &ProgressBarConfig<T>), Added<T>>,
    transform_query: Query<&Transform>,
) {
    for (entity, percentage, config) in query.iter() {
        // spawn progress bar
        let foreground_color = materials.add(config.color(percentage));
        let foreground_color_moderate = materials.add(config.color_for_state(&BarState::Moderate));
        let foreground_color_critical = materials.add(config.color_for_state(&BarState::Critical));
        let background_color = materials.add(config.background_color());

        let Ok(entity_transform) = transform_query.get(entity) else {
            tracing::warn!("Failed to get parent transform for progress bar");
            continue;
        };

        let bar_background = commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(config.background_mesh()).into(),
                material: background_color,
                transform: config.background_transform(entity_transform),
                ..default()
            })
            .id();
        let bar_foreground = commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(config.foreground_mesh(percentage)).into(),
                material: foreground_color.clone(),
                transform: config.foreground_transform(entity_transform, percentage),
                ..default()
            })
            .id();

        commands
            .entity(entity)
            .insert(WithProgressBar::<T> {
                foreground: bar_foreground,
                background: bar_background,
                ok_handle: foreground_color,
                moderate_handle: foreground_color_moderate,
                critical_handle: foreground_color_critical,
                _marker: std::marker::PhantomData,
            })
            .add_child(bar_background)
            .add_child(bar_foreground);
    }
}

/// System to update progress bars for entities with a `Percentage` component.
/// This system is run on every frame.
/// The `Changed<T>` query gets entities which have the `T` component and have changed.
///
/// This system updates the progress bar's foreground mesh to match the percentage.
///
/// # Parameters
///
/// * `meshes`: The mesh asset storage. This is used because we update a mesh with a new one for the foreground.
/// Todo: This probably is terribly inefficient, since if there's a lot of progress bars.. potentially a lot of meshes are being created.
/// * `parent_query`: The query for the parent entity, the entity with the `Percentage` component. We also grab the
/// `ProgressBarConfig` component to get the configuration for the progress bar, and the `WithProgressBar` component
/// to get the progress bar entities. Finally, we grab the children of the parent entity because we spawn the progress
/// bar entities as children of the parent entity.
/// * `mesh_query`: The query for the progress bar entities. We grab the mesh and transform components to update the mesh and transform.
#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
fn update<T: Percentage + Component>(
    mut commands: Commands,
    parent_query: Query<(&T, &ProgressBarConfig<T>, &WithProgressBar<T>, &Children), Changed<T>>,
    mut mesh_query: Query<&mut Transform>,
) {
    parent_query
        .iter()
        .for_each(|(percentage, config, progress_bar, children)| {
            let (_, foreground) = progress_bar.get();

            // loop over child entity ids
            for child in children {
                // if this isn't the foreground entity, skip it
                if *child != foreground {
                    continue;
                }

                tracing::info!(
                    "Updating progress bar for entity with percentage {}",
                    percentage.percentage()
                );

                // grab the mutable mesh
                let Ok(mut transform) = mesh_query.get_mut(*child) else {
                    tracing::warn!(
                        "Failed to get foreground mesh for progress bar Entity {foreground:?}"
                    );
                    return;
                };

                // update the mesh
                commands
                    .entity(*child)
                    .insert(progress_bar.get_material(config, percentage));
                // update the transform
                transform.scale = Vec3::new(percentage.percentage(), 1.0, 1.0);
                // update the translation.x to be the percentage of the parent's width
                transform.translation.x =
                    config.position_translation.x + (percentage.percentage() * config.size.x / 2.0);
            }
        });
}

#[allow(clippy::needless_pass_by_value)]
fn remove<T: Percentage + Component>(
    mut commands: Commands,
    mut removals: RemovedComponents<T>,
    parent_query: Query<&WithProgressBar<T>>,
) {
    removals.read().for_each(|entity| {
        let Ok(&WithProgressBar {
            foreground,
            background,
            ..
        }) = parent_query.get(entity)
        else {
            return;
        };

        commands.entity(foreground).despawn_recursive();
        commands.entity(background).despawn_recursive();
    });
}
