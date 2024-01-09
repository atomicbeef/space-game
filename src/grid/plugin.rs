use bevy::prelude::*;

use crate::app_setup::AssetInitialization;

use super::chunk::ChunkChanged;
use super::collider::regenerate_chunk_colliders;
use super::mesh::regenerate_chunk_meshes;
use super::GridMaterialHandle;

fn init_grid_material(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut grid_material_handle: ResMut<GridMaterialHandle>,
) {
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("aluminum/albedo.png")),
        metallic: 1.0,
        perceptual_roughness: 0.79,
        normal_map_texture: Some(asset_server.load("aluminum/normal.png")),
        ..Default::default()
    });

    grid_material_handle.0 = material_handle;
}

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChunkChanged>()
            .insert_resource(GridMaterialHandle(Handle::default()))
            .add_systems(Startup, init_grid_material.in_set(AssetInitialization))
            .add_systems(
                FixedUpdate,
                (regenerate_chunk_meshes, regenerate_chunk_colliders),
            );
    }
}
