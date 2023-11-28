use bevy::prelude::*;
use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};

use super::{Grid, GridPos, GridChanged};
use crate::block::BLOCK_SIZE;

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
    index_offset: &mut u32
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
        *index_offset, *index_offset + 1, *index_offset + 2, *index_offset + 2, *index_offset + 3, *index_offset
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
    index_offset: &mut u32
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
        *index_offset, *index_offset + 1, *index_offset + 2, *index_offset + 2, *index_offset + 3, *index_offset
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
    index_offset: &mut u32
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
        *index_offset, *index_offset + 1, *index_offset + 2, *index_offset + 2, *index_offset + 3, *index_offset
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
    index_offset: &mut u32
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
        *index_offset, *index_offset + 1, *index_offset + 2, *index_offset + 2, *index_offset + 3, *index_offset
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
    index_offset: &mut u32
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
        *index_offset, *index_offset + 1, *index_offset + 2, *index_offset + 2, *index_offset + 3, *index_offset
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
    index_offset: &mut u32
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
        *index_offset, *index_offset + 1, *index_offset + 2, *index_offset + 2, *index_offset + 3, *index_offset
    ]);
    *index_offset += 4;
}

pub fn generate_grid_mesh(grid: &Grid) -> Mesh {
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut triangles: Vec<u32> = Vec::new();
    let mut index_offset = 0;

    for (grid_pos, _) in grid.blocks.iter() {
        let x = grid_pos.x as f32 * BLOCK_SIZE;
        let y = grid_pos.y as f32 * BLOCK_SIZE;
        let z = grid_pos.z as f32 * BLOCK_SIZE;

        if grid.blocks.get(&GridPos::new(grid_pos.x + 1, grid_pos.y, grid_pos.z)).is_none() {
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
                &mut index_offset
            );
        } if grid.blocks.get(&GridPos::new(grid_pos.x - 1, grid_pos.y, grid_pos.z)).is_none() {
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
                &mut index_offset
            );
        } if grid.blocks.get(&GridPos::new(grid_pos.x, grid_pos.y + 1, grid_pos.z)).is_none() {
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
                &mut index_offset
            );
        } if grid.blocks.get(&GridPos::new(grid_pos.x, grid_pos.y - 1, grid_pos.z)).is_none() {
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
                &mut index_offset
            );
        } if grid.blocks.get(&GridPos::new(grid_pos.x, grid_pos.y, grid_pos.z + 1)).is_none() {
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
                &mut index_offset
            );
        } if grid.blocks.get(&GridPos::new(grid_pos.x, grid_pos.y, grid_pos.z - 1)).is_none() {
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
                &mut index_offset
            );
        }
    }
    
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(triangles)));

    mesh
}

pub fn regenerate_grid_meshes(
    mut grid_changed_events: EventReader<GridChanged>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grid_query: Query<&Grid>,
    mut commands: Commands,
) {
    for grid_changed in grid_changed_events.read() {
        let Ok(grid) = grid_query.get(grid_changed.0) else {
            return;
        };

        let mesh = generate_grid_mesh(grid);
        let mesh_handle = meshes.add(mesh);
        let material_handle = materials.add(Color::rgb(0.5, 0.5, 0.5).into());

        commands.entity(grid_changed.0).insert((mesh_handle, material_handle));
    }
}