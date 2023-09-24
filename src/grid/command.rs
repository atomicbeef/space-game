use bevy::prelude::*;
use bevy::ecs::system::{Command, SystemState};

use super::Grid;
use super::mesh::generate_grid_mesh;

pub struct SpawnGrid {
    pub transform: Transform,
    pub grid: Grid,
}

impl Command for SpawnGrid {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(
            ResMut<Assets<Mesh>>,
            ResMut<Assets<StandardMaterial>>,
            Commands,
        )> = SystemState::new(world);

        let (
            mut meshes,
            mut materials,
            mut commands,
        ) = system_state.get_mut(world);

        let mesh = generate_grid_mesh(&self.grid);
        let mesh_handle = meshes.add(mesh);
        let material_handle = materials.add(Color::rgb(0.5, 0.5, 0.5).into());

        commands.spawn((
            PbrBundle {
                mesh: mesh_handle,
                material: material_handle,
                transform: self.transform,
                ..Default::default()
            },
            self.grid,
        ));

        system_state.apply(world);
    }
}

impl SpawnGrid {
    pub fn new(transform: Transform, grid: Grid) -> Self {
        Self {
            transform,
            grid,
        }
    }
}