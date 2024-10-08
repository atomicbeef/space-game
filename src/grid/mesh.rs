use bevy::prelude::*;
use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};
use bevy::render::primitives::Aabb;

use super::block::BlockMaterial;
use super::chunk::{Chunk, ChunkChanged, CHUNK_SIZE};
use super::{ChunkPos, Grid};
use crate::grid::block::BLOCK_SIZE;

fn add_right_face(
    x: f32,
    min_y: f32,
    max_y: f32,
    min_z: f32,
    max_z: f32,
    vertices: &mut Vec<[f32; 3]>,
    triangles: &mut Vec<u32>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    index_offset: &mut u32,
) {
    let verts = &[
        ([x, min_y, min_z], [1.0, 0., 0.], [0., 0.]),
        ([x, max_y, min_z], [1.0, 0., 0.], [1.0, 0.]),
        ([x, max_y, max_z], [1.0, 0., 0.], [1.0, 1.0]),
        ([x, min_y, max_z], [1.0, 0., 0.], [0., 1.0]),
    ];

    vertices.extend(verts.iter().map(|(p, _, _)| *p));
    normals.extend(verts.iter().map(|(_, n, _)| *n));
    uvs.extend(verts.iter().map(|(_, _, uv)| *uv));

    triangles.extend([
        *index_offset,
        *index_offset + 1,
        *index_offset + 2,
        *index_offset + 2,
        *index_offset + 3,
        *index_offset,
    ]);
    *index_offset += 4;
}

fn add_left_face(
    x: f32,
    min_y: f32,
    max_y: f32,
    min_z: f32,
    max_z: f32,
    vertices: &mut Vec<[f32; 3]>,
    triangles: &mut Vec<u32>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    index_offset: &mut u32,
) {
    let verts = &[
        ([x, min_y, max_z], [-1.0, 0., 0.], [1.0, 0.]),
        ([x, max_y, max_z], [-1.0, 0., 0.], [0., 0.]),
        ([x, max_y, min_z], [-1.0, 0., 0.], [0., 1.0]),
        ([x, min_y, min_z], [-1.0, 0., 0.], [1.0, 1.0]),
    ];

    vertices.extend(verts.iter().map(|(p, _, _)| *p));
    normals.extend(verts.iter().map(|(_, n, _)| *n));
    uvs.extend(verts.iter().map(|(_, _, uv)| *uv));

    triangles.extend([
        *index_offset,
        *index_offset + 1,
        *index_offset + 2,
        *index_offset + 2,
        *index_offset + 3,
        *index_offset,
    ]);
    *index_offset += 4;
}

fn add_top_face(
    min_x: f32,
    max_x: f32,
    y: f32,
    min_z: f32,
    max_z: f32,
    vertices: &mut Vec<[f32; 3]>,
    triangles: &mut Vec<u32>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    index_offset: &mut u32,
) {
    let verts = &[
        ([max_x, y, min_z], [0., 1.0, 0.], [1.0, 0.]),
        ([min_x, y, min_z], [0., 1.0, 0.], [0., 0.]),
        ([min_x, y, max_z], [0., 1.0, 0.], [0., 1.0]),
        ([max_x, y, max_z], [0., 1.0, 0.], [1.0, 1.0]),
    ];

    vertices.extend(verts.iter().map(|(p, _, _)| *p));
    normals.extend(verts.iter().map(|(_, n, _)| *n));
    uvs.extend(verts.iter().map(|(_, _, uv)| *uv));

    triangles.extend([
        *index_offset,
        *index_offset + 1,
        *index_offset + 2,
        *index_offset + 2,
        *index_offset + 3,
        *index_offset,
    ]);
    *index_offset += 4;
}

fn add_bottom_face(
    min_x: f32,
    max_x: f32,
    y: f32,
    min_z: f32,
    max_z: f32,
    vertices: &mut Vec<[f32; 3]>,
    triangles: &mut Vec<u32>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    index_offset: &mut u32,
) {
    let verts = &[
        ([max_x, y, max_z], [0., -1.0, 0.], [0., 0.]),
        ([min_x, y, max_z], [0., -1.0, 0.], [1.0, 0.]),
        ([min_x, y, min_z], [0., -1.0, 0.], [1.0, 1.0]),
        ([max_x, y, min_z], [0., -1.0, 0.], [0., 1.0]),
    ];

    vertices.extend(verts.iter().map(|(p, _, _)| *p));
    normals.extend(verts.iter().map(|(_, n, _)| *n));
    uvs.extend(verts.iter().map(|(_, _, uv)| *uv));

    triangles.extend([
        *index_offset,
        *index_offset + 1,
        *index_offset + 2,
        *index_offset + 2,
        *index_offset + 3,
        *index_offset,
    ]);
    *index_offset += 4;
}

fn add_front_face(
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
    z: f32,
    vertices: &mut Vec<[f32; 3]>,
    triangles: &mut Vec<u32>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    index_offset: &mut u32,
) {
    let verts = &[
        ([min_x, min_y, z], [0., 0., 1.0], [0., 0.]),
        ([max_x, min_y, z], [0., 0., 1.0], [1.0, 0.]),
        ([max_x, max_y, z], [0., 0., 1.0], [1.0, 1.0]),
        ([min_x, max_y, z], [0., 0., 1.0], [0., 1.0]),
    ];

    vertices.extend(verts.iter().map(|(p, _, _)| *p));
    normals.extend(verts.iter().map(|(_, n, _)| *n));
    uvs.extend(verts.iter().map(|(_, _, uv)| *uv));

    triangles.extend([
        *index_offset,
        *index_offset + 1,
        *index_offset + 2,
        *index_offset + 2,
        *index_offset + 3,
        *index_offset,
    ]);
    *index_offset += 4;
}

fn add_back_face(
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
    z: f32,
    vertices: &mut Vec<[f32; 3]>,
    triangles: &mut Vec<u32>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    index_offset: &mut u32,
) {
    let verts = &[
        ([min_x, max_y, z], [0., 0., -1.0], [1.0, 0.]),
        ([max_x, max_y, z], [0., 0., -1.0], [0., 0.]),
        ([max_x, min_y, z], [0., 0., -1.0], [0., 1.0]),
        ([min_x, min_y, z], [0., 0., -1.0], [1.0, 1.0]),
    ];

    vertices.extend(verts.iter().map(|(p, _, _)| *p));
    normals.extend(verts.iter().map(|(_, n, _)| *n));
    uvs.extend(verts.iter().map(|(_, _, uv)| *uv));

    triangles.extend([
        *index_offset,
        *index_offset + 1,
        *index_offset + 2,
        *index_offset + 2,
        *index_offset + 3,
        *index_offset,
    ]);
    *index_offset += 4;
}

pub fn generate_chunk_mesh(chunk: &Chunk) -> Mesh {
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut triangles: Vec<u32> = Vec::new();
    let mut index_offset = 0;

    for c_z in 0..CHUNK_SIZE {
        for c_y in 0..CHUNK_SIZE {
            for c_x in 0..CHUNK_SIZE {
                if chunk.get(c_x, c_y, c_z).material == BlockMaterial::Empty {
                    continue;
                }

                let x = c_x as f32 * BLOCK_SIZE;
                let y = c_y as f32 * BLOCK_SIZE;
                let z = c_z as f32 * BLOCK_SIZE;

                if c_x == CHUNK_SIZE - 1
                    || chunk.get(c_x + 1, c_y, c_z).material == BlockMaterial::Empty
                {
                    add_right_face(
                        x + BLOCK_SIZE,
                        y,
                        y + BLOCK_SIZE,
                        z,
                        z + BLOCK_SIZE,
                        &mut vertices,
                        &mut triangles,
                        &mut normals,
                        &mut uvs,
                        &mut index_offset,
                    );
                }

                if c_x == 0 || chunk.get(c_x - 1, c_y, c_z).material == BlockMaterial::Empty {
                    add_left_face(
                        x,
                        y,
                        y + BLOCK_SIZE,
                        z,
                        z + BLOCK_SIZE,
                        &mut vertices,
                        &mut triangles,
                        &mut normals,
                        &mut uvs,
                        &mut index_offset,
                    );
                }

                if c_y == CHUNK_SIZE - 1
                    || chunk.get(c_x, c_y + 1, c_z).material == BlockMaterial::Empty
                {
                    add_top_face(
                        x,
                        x + BLOCK_SIZE,
                        y + BLOCK_SIZE,
                        z,
                        z + BLOCK_SIZE,
                        &mut vertices,
                        &mut triangles,
                        &mut normals,
                        &mut uvs,
                        &mut index_offset,
                    );
                }

                if c_y == 0 || chunk.get(c_x, c_y - 1, c_z).material == BlockMaterial::Empty {
                    add_bottom_face(
                        x,
                        x + BLOCK_SIZE,
                        y,
                        z,
                        z + BLOCK_SIZE,
                        &mut vertices,
                        &mut triangles,
                        &mut normals,
                        &mut uvs,
                        &mut index_offset,
                    );
                }

                if c_z == CHUNK_SIZE - 1
                    || chunk.get(c_x, c_y, c_z + 1).material == BlockMaterial::Empty
                {
                    add_front_face(
                        x,
                        x + BLOCK_SIZE,
                        y,
                        y + BLOCK_SIZE,
                        z + BLOCK_SIZE,
                        &mut vertices,
                        &mut triangles,
                        &mut normals,
                        &mut uvs,
                        &mut index_offset,
                    );
                }

                if c_z == 0 || chunk.get(c_x, c_y, c_z - 1).material == BlockMaterial::Empty {
                    add_back_face(
                        x,
                        x + BLOCK_SIZE,
                        y,
                        y + BLOCK_SIZE,
                        z,
                        &mut vertices,
                        &mut triangles,
                        &mut normals,
                        &mut uvs,
                        &mut index_offset,
                    );
                }
            }
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(triangles)));
    mesh.generate_tangents().unwrap();

    mesh
}

pub fn regenerate_chunk_meshes(
    mut chunk_changed_events: EventReader<ChunkChanged>,
    mut meshes: ResMut<Assets<Mesh>>,
    chunk_query: Query<(&ChunkPos, &Parent)>,
    grid_query: Query<&Grid>,
    mut commands: Commands,
) {
    for chunk_changed in chunk_changed_events.read() {
        let Ok((chunk_pos, parent)) = chunk_query.get(chunk_changed.0) else {
            continue;
        };

        let grid_entity = parent.get();

        let Ok(grid) = grid_query.get(grid_entity) else {
            return;
        };

        let Some(chunk) = grid.get_chunk(*chunk_pos) else {
            return;
        };

        let mesh = generate_chunk_mesh(chunk);
        let mesh_handle = meshes.add(mesh);
        commands
            .entity(chunk_changed.0)
            .insert(mesh_handle)
            // Work around mesh AABBs not being updated automatically: https://github.com/bevyengine/bevy/issues/4294
            .remove::<Aabb>();
    }
}
