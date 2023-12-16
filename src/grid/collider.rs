use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::block::{BLOCK_SIZE, BlockMaterial};

use super::chunk::{Chunk, CHUNK_SIZE, CHUNK_SIZE_CUBED};
use super::{Grid, ChunkChanged, ChunkPos};

pub fn generate_collider_for_chunk(chunk: &Chunk) -> Collider {
    let mut collider_data: Vec<(Vec3, Quat, Collider)> = Vec::new();
    let mut tested = vec![false; CHUNK_SIZE_CUBED];

    for start_z in 0..CHUNK_SIZE {
        for start_y in 0..CHUNK_SIZE {
            for start_x in 0..CHUNK_SIZE {
                let start_index = chunk.pos_to_index(start_x, start_y, start_z);
                if tested[start_index] {
                    continue; 
                }

                let block = chunk.get(start_x, start_y, start_z);

                if matches!(block.material, BlockMaterial::Empty) {
                    tested[start_index] = true;
                    continue;
                }

                tested[start_index] = true;

                let mut end_x = start_x;
                let mut end_y = start_y;
                let mut end_z = start_z;

                for x in start_x + 1..CHUNK_SIZE {
                    let current_index = chunk.pos_to_index(x, start_y, start_z);
                    let test_block = chunk.blocks()[current_index];

                    if test_block.material != block.material || tested[current_index] {
                        end_x = x - 1;
                        break;
                    }

                    if x == CHUNK_SIZE - 1 {
                        end_x = x;
                    }

                    tested[current_index] = true;
                }

                'height: for y in start_y + 1..CHUNK_SIZE {
                    for x in start_x..end_x + 1 {
                        let current_index = chunk.pos_to_index(x, y, start_z);
                        let test_block = chunk.blocks()[current_index];

                        if test_block.material != block.material || tested[current_index] {
                            end_y = y - 1;
                            break 'height;
                        }
                    }

                    for x in start_x..end_x + 1 {
                        tested[chunk.pos_to_index(x, y, start_z)] = true;
                    }

                    if y == CHUNK_SIZE - 1 {
                        end_y = y;
                    }
                }

                'depth: for z in start_z + 1..CHUNK_SIZE {
                    for y in start_y..end_y + 1 {
                        for x in start_x..end_x + 1 {
                            let current_index = chunk.pos_to_index(x, y, z);
                            let test_block = chunk.blocks()[current_index];
                            if test_block.material != block.material || tested[current_index] {
                                end_z = z - 1;
                                break 'depth;
                            }
                        }
                    }

                    for y in start_y..end_y + 1 {
                        for x in start_x..end_x + 1 {
                            tested[chunk.pos_to_index(x, y, z)] = true;
                        }
                    }

                    if z == CHUNK_SIZE - 1 {
                        end_z = z;
                    }
                }

                let hx = (end_x + 1 - start_x) as f32 / 2.0 * BLOCK_SIZE;
                let hy = (end_y + 1 - start_y) as f32 / 2.0 * BLOCK_SIZE;
                let hz = (end_z + 1 - start_z) as f32 / 2.0 * BLOCK_SIZE;

                let collider = Collider::cuboid(hx, hy, hz);

                collider_data.push((
                    Vec3::new(start_x as f32, start_y as f32, start_z as f32) * BLOCK_SIZE + Vec3::new(hx, hy, hz),
                    Quat::IDENTITY,
                    collider
                ));
            }
        }
    }

    Collider::compound(collider_data)
}

pub fn regenerate_chunk_colliders(
    mut chunk_changed_events: EventReader<ChunkChanged>,
    grid_query: Query<(&Grid, &Children)>,
    chunk_query: Query<&ChunkPos>,
    mut commands: Commands,
) {
    for chunk_changed in chunk_changed_events.read() {
        let Ok((grid, children)) = grid_query.get(chunk_changed.grid_entity) else {
            return;
        };

        let Some(chunk) = grid.get_chunk(chunk_changed.chunk_pos) else {
            return;
        };
        
        for &child in children.iter() {
            if let Ok(&pos) = chunk_query.get(child) {
                if pos == chunk_changed.chunk_pos {
                    let collider = generate_collider_for_chunk(chunk);
                    commands.entity(child).insert(collider);
                    
                    break;
                }
            }
        }
    }
}