use bevy::ecs::system::{Command, SystemState};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::building_material::BuildingMaterialHandle;
use crate::UniverseGrid;

use super::block::BLOCK_SIZE;
use super::collider::generate_collider_for_chunk;
use super::mesh::generate_chunk_mesh;
use super::Grid;

pub struct SpawnGrid {
    pub transform: Transform,
    pub grid: Grid,
}

impl Command for SpawnGrid {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(
            ResMut<Assets<Mesh>>,
            Res<BuildingMaterialHandle>,
            Commands,
        )> = SystemState::new(world);

        let (mut meshes, material_handle, mut commands) = system_state.get_mut(world);

        let mut chunk_entities = Vec::with_capacity(self.grid.chunks.len());

        for (pos, chunk) in self.grid.chunks.iter() {
            let mesh = generate_chunk_mesh(chunk);
            let mesh_handle = meshes.add(mesh);
            let collider = generate_collider_for_chunk(chunk);

            let entity = commands
                .spawn((
                    MaterialMeshBundle {
                        mesh: mesh_handle,
                        material: material_handle.0.clone().unwrap(),
                        transform: Transform::from_translation(Vec3::new(
                            pos.x as f32 * BLOCK_SIZE,
                            pos.y as f32 * BLOCK_SIZE,
                            pos.z as f32 * BLOCK_SIZE,
                        )),
                        ..Default::default()
                    },
                    collider,
                    *pos,
                ))
                .id();

            chunk_entities.push(entity);
        }

        commands
            .spawn((
                SpatialBundle {
                    transform: self.transform,
                    ..Default::default()
                },
                self.grid,
                RigidBody::Dynamic,
                Ccd::enabled(),
                UniverseGrid::default(),
            ))
            .push_children(&chunk_entities);

        system_state.apply(world);
    }
}

impl SpawnGrid {
    pub fn new(transform: Transform, grid: Grid) -> Self {
        Self { transform, grid }
    }
}
