use bevy::prelude::*;

use crate::fixed_update::AddFixedEvent;

use super::collider::regenerate_chunk_colliders;
use super::mesh::regenerate_chunk_meshes;
use super::ChunkChanged;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_fixed_event::<ChunkChanged>().add_systems(
            FixedUpdate,
            (regenerate_chunk_meshes, regenerate_chunk_colliders),
        );
    }
}
