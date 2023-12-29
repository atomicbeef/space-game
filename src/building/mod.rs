use bevy::prelude::*;
use big_space::FloatingOrigin;

use crate::building_material::BuildingMaterialHandle;
use crate::camera::ActiveCamera;
use crate::fixed_update::{FixedInput, FixedUpdateSet};
use crate::grid::block::{Block, BlockMaterial, BLOCK_SIZE};
use crate::grid::chunk::{BlockPos, Chunk, ChunkBundle, ChunkChanged};
use crate::grid::command::DespawnChunk;
use crate::grid::{ChunkPos, Grid, GridPos};
use crate::raycast_selection::SelectionSource;
use crate::UniverseGrid;

use self::events::PlaceBlockRequest;

pub mod events;

#[derive(Component)]
pub struct BuildMarker;

fn snap_to_grid(point: Vec3, snap_resolution: f32) -> Vec3 {
    // This extra rounding smoothes out any jittering
    let rounded_x = (point.x * 1000.0).round();
    let rounded_y = (point.y * 1000.0).round();
    let rounded_z = (point.z * 1000.0).round();

    let x = (rounded_x * 1.0 / (snap_resolution * 1000.0)).floor() / (1.0 / snap_resolution);
    let y = (rounded_y * 1.0 / (snap_resolution * 1000.0)).floor() / (1.0 / snap_resolution);
    let z = (rounded_z * 1.0 / (snap_resolution * 1000.0)).floor() / (1.0 / snap_resolution);

    Vec3::new(x, y, z)
}

fn move_build_marker(
    mut build_marker_query: Query<
        (&mut Visibility, &mut Transform, &mut UniverseGrid),
        (With<BuildMarker>, Without<FloatingOrigin>),
    >,
    selection_source_query: Query<&SelectionSource, With<ActiveCamera>>,
    floating_origin_query: Query<&UniverseGrid, With<FloatingOrigin>>,
    global_transform_query: Query<&GlobalTransform>,
) {
    let Ok((
        mut build_marker_visibility,
        mut build_marker_transform,
        mut build_marker_universe_grid,
    )) = build_marker_query.get_single_mut()
    else {
        return;
    };

    let Ok(selection_source) = selection_source_query.get_single() else {
        *build_marker_visibility = Visibility::Hidden;
        return;
    };

    let Some((chunk_entity, intersection)) = selection_source.intersection() else {
        *build_marker_visibility = Visibility::Hidden;
        return;
    };

    let Ok(floating_origin) = floating_origin_query.get_single() else {
        *build_marker_visibility = Visibility::Hidden;
        return;
    };

    let chunk_transform = global_transform_query.get(chunk_entity).unwrap();
    let chunk_transform_affine = chunk_transform.affine();
    let chunk_transform_inverse = chunk_transform_affine.inverse();

    let inverse_normal = chunk_transform_inverse.transform_vector3(intersection.normal);

    let block_pos = snap_to_grid(
        chunk_transform_inverse.transform_point(intersection.point),
        BLOCK_SIZE,
    ) + Vec3::splat(BLOCK_SIZE / 2.0) * (Vec3::splat(1.0) - inverse_normal.abs())
        + inverse_normal * BLOCK_SIZE / 2.0;

    *build_marker_universe_grid = *floating_origin;
    build_marker_transform.translation = chunk_transform_affine.transform_point(block_pos);
    build_marker_transform.rotation = chunk_transform.to_scale_rotation_translation().1;
    *build_marker_visibility = Visibility::Visible;
}

fn create_build_request_events(
    mouse_buttons: Res<FixedInput<MouseButton>>,
    mut place_block_requests: EventWriter<PlaceBlockRequest>,
    selection_source_query: Query<&SelectionSource, With<ActiveCamera>>,
    chunk_query: Query<(&GlobalTransform, &ChunkPos, &Parent)>,
) {
    let Ok(selection_source) = selection_source_query.get_single() else {
        return;
    };
    let Some((chunk_entity, intersection)) = selection_source.intersection() else {
        return;
    };

    let Ok((chunk_transform, chunk_pos, chunk_parent)) = chunk_query.get(chunk_entity) else {
        return;
    };

    let chunk_transform_affine = chunk_transform.affine();
    let chunk_transform_inverse = chunk_transform_affine.inverse();

    let inverse_normal = chunk_transform_inverse.transform_vector3(intersection.normal);

    let snapped_intersection = snap_to_grid(
        chunk_transform_inverse.transform_point(intersection.point),
        BLOCK_SIZE,
    ) / BLOCK_SIZE
        - inverse_normal * 0.5;

    let block_pos = BlockPos {
        x: snapped_intersection.x as u8,
        y: snapped_intersection.y as u8,
        z: snapped_intersection.z as u8,
    };

    if mouse_buttons.just_pressed(MouseButton::Left) {
        let selected_pos = GridPos {
            chunk_pos: *chunk_pos,
            block_pos,
        };

        let new_block_pos = selected_pos
            + (
                inverse_normal.x as i16,
                inverse_normal.y as i16,
                inverse_normal.z as i16,
            );

        place_block_requests.send(PlaceBlockRequest {
            grid: chunk_parent.get(),
            pos: new_block_pos,
            block: Block {
                material: BlockMaterial::Aluminum,
            },
        })
    } else if mouse_buttons.just_pressed(MouseButton::Right) {
        let selected_pos = GridPos {
            chunk_pos: *chunk_pos,
            block_pos,
        };

        place_block_requests.send(PlaceBlockRequest {
            grid: chunk_parent.get(),
            pos: selected_pos,
            block: Block {
                material: BlockMaterial::Empty,
            },
        })
    }
}

struct DeleteChunkData {
    pub chunk_entity: Entity,
}

fn place_blocks(
    mut dirty_chunks: Local<Vec<Entity>>,
    mut chunks_to_delete: Local<Vec<DeleteChunkData>>,
    mut place_block_requests: EventReader<PlaceBlockRequest>,
    mut grid_query: Query<&mut Grid>,
    mut commands: Commands,
    building_material_handle: Res<BuildingMaterialHandle>,
    mut chunk_changed_writer: EventWriter<ChunkChanged>,
) {
    dirty_chunks.clear();
    chunks_to_delete.clear();

    for request in place_block_requests.read() {
        let Ok(mut grid) = grid_query.get_mut(request.grid) else {
            continue;
        };

        if request.block.material == BlockMaterial::Empty {
            if let Some(chunk) = grid.get_chunk_mut(request.pos.chunk_pos) {
                chunk.set_by_block_pos(
                    request.pos.block_pos,
                    Block {
                        material: BlockMaterial::Empty,
                    },
                );

                if chunk
                    .blocks()
                    .iter()
                    .all(|block| block.material == BlockMaterial::Empty)
                {
                    chunks_to_delete.push(DeleteChunkData {
                        chunk_entity: chunk.entity,
                    });
                } else {
                    if !dirty_chunks.contains(&chunk.entity) {
                        dirty_chunks.push(chunk.entity);
                    }
                }
            } else {
                continue;
            }
        } else {
            if let Some(chunk) = grid.get_chunk_mut(request.pos.chunk_pos) {
                chunk.set_by_block_pos(request.pos.block_pos, request.block);

                if !dirty_chunks.contains(&chunk.entity) {
                    dirty_chunks.push(chunk.entity);
                }
            } else {
                let chunk_entity = commands
                    .spawn(ChunkBundle::new(
                        request.pos.chunk_pos,
                        building_material_handle.0.clone().unwrap(),
                    ))
                    .id();
                commands.entity(request.grid).add_child(chunk_entity);

                let mut chunk = Chunk::new(
                    chunk_entity,
                    [Block {
                        material: BlockMaterial::Empty,
                    }; 4096],
                );
                chunk.set_by_block_pos(request.pos.block_pos, request.block);

                if !dirty_chunks.contains(&chunk_entity) {
                    dirty_chunks.push(chunk_entity);
                }

                grid.set_chunk(request.pos.chunk_pos, Some(chunk));
            }
        }
    }

    for chunk_data in chunks_to_delete.iter() {
        commands.add(DespawnChunk {
            entity: chunk_data.chunk_entity,
        });
    }

    chunk_changed_writer.send_batch(dirty_chunks.iter().copied().map(ChunkChanged));
}

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaceBlockRequest>()
            .add_systems(Update, create_build_request_events)
            .add_systems(
                FixedUpdate,
                (move_build_marker, place_blocks)
                    .chain()
                    .in_set(FixedUpdateSet::Update),
            );
    }
}
