use bevy::prelude::*;

use super::chunk::ChunkChanged;
use super::collider::regenerate_chunk_colliders;
use super::mesh::regenerate_chunk_meshes;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChunkChanged>().add_systems(
            FixedUpdate,
            (regenerate_chunk_meshes, regenerate_chunk_colliders),
        );
    }
}
