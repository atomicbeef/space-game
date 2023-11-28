use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::block::BLOCK_SIZE;

use super::{Grid, GridChanged};

pub fn generate_colliders_for_grid(grid: &Grid) -> Collider {
    let collider_data: Vec<(Vec3, Quat, Collider)> = grid.blocks.keys().map(|pos| {
        (
            Vec3::new(
                pos.x as f32 * BLOCK_SIZE + BLOCK_SIZE / 2.0,
                pos.y as f32 * BLOCK_SIZE + BLOCK_SIZE / 2.0,
                pos.z as f32 * BLOCK_SIZE + BLOCK_SIZE / 2.0,
            ),
            Quat::IDENTITY,
            Collider::cuboid(BLOCK_SIZE / 2.0, BLOCK_SIZE / 2.0, BLOCK_SIZE / 2.0)
        )
    }).collect();

    Collider::compound(collider_data)
}

pub fn regenerate_grid_colliders(
    mut grid_changed_events: EventReader<GridChanged>,
    grid_query: Query<&Grid>,
    mut commands: Commands,
) {
    for grid_changed in grid_changed_events.read() {
        let Ok(grid) = grid_query.get(grid_changed.0) else {
            return;
        };

        let collider = generate_colliders_for_grid(grid);

        commands.entity(grid_changed.0).insert(collider);
    }
}