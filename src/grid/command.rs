use bevy::ecs::system::{Command, SystemState};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::building_material::BuildingMaterialHandle;
use crate::UniverseGrid;

use super::chunk::ChunkBundle;
use super::collider::generate_collider_for_chunk;
use super::mesh::generate_chunk_mesh;
use super::{ChunkPos, Grid};

pub struct SpawnGrid {
    pub transform: Transform,
    pub grid: Grid,
}

impl Command for SpawnGrid {
    fn apply(mut self, world: &mut World) {
        let mut system_state: SystemState<(
            ResMut<Assets<Mesh>>,
            Res<BuildingMaterialHandle>,
            Commands,
        )> = SystemState::new(world);

        let (mut meshes, material_handle, mut commands) = system_state.get_mut(world);

        let mut chunk_entities = Vec::with_capacity(self.grid.chunks.len());

        for (pos, chunk) in self.grid.chunks.iter_mut() {
            let mesh = generate_chunk_mesh(chunk);
            let mesh_handle = meshes.add(mesh);
            let collider = generate_collider_for_chunk(chunk);

            let entity = commands
                .spawn((
                    ChunkBundle::new(*pos, material_handle.0.clone().unwrap()),
                    mesh_handle,
                    collider,
                ))
                .id();

            chunk_entities.push(entity);

            chunk.entity = entity;
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
                TransformInterpolation::default(),
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

pub struct DespawnChunk {
    pub entity: Entity,
}

impl Command for DespawnChunk {
    fn apply(self, world: &mut World) {
        let Some(&chunk_pos) = world
            .get_entity(self.entity)
            .and_then(|entity| entity.get::<ChunkPos>())
        else {
            return;
        };

        let grid_entity = world.entity(self.entity).get::<Parent>().unwrap().get();

        world
            .entity_mut(grid_entity)
            .remove_children(&[self.entity]);
        world.entity_mut(self.entity).despawn_recursive();

        let mut grid_commands = world.entity_mut(grid_entity);
        let mut grid = grid_commands.get_mut::<Grid>().unwrap();
        grid.chunks.remove(&chunk_pos);

        if grid.chunks.is_empty() {
            grid_commands.despawn_recursive();
        }
    }
}
