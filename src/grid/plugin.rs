use bevy::prelude::*;

use super::GridChanged;
use super::mesh::regenerate_grid_meshes;
use super::collider::regenerate_grid_colliders;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GridChanged>()
            .add_systems(FixedUpdate, (
                regenerate_grid_meshes,
                regenerate_grid_colliders,
            ));
    }
}