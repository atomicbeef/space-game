use bevy::prelude::*;

use super::GridChanged;
use super::mesh::regenerate_grid_meshes;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GridChanged>()
            .add_systems(FixedUpdate, (
                regenerate_grid_meshes,
            ));
    }
}