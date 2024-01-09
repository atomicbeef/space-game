use bevy::input::common_conditions::input_just_pressed;
use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

use crate::app_setup::AssetInitialization;
use crate::camera::ActiveCamera;
use crate::grid::{ChunkPos, GridMaterialHandle};
use crate::raycast_selection::SelectionSource;

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct BuildingMaterial {
    #[uniform(100)]
    grid_color: Color,
}

impl BuildingMaterial {
    pub fn new(grid_color: Color) -> Self {
        Self { grid_color }
    }
}

impl Default for BuildingMaterial {
    fn default() -> Self {
        Self::new(Color::BLACK)
    }
}

impl MaterialExtension for BuildingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/building_material.wgsl".into()
    }
}

type BuildingMaterialType = ExtendedMaterial<StandardMaterial, BuildingMaterial>;

#[derive(Resource)]
pub struct BuildingMaterialHandle(pub Handle<BuildingMaterialType>);

fn init_building_material(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<BuildingMaterialType>>,
    mut building_material_handle: ResMut<BuildingMaterialHandle>,
) {
    let material_handle = materials.add(ExtendedMaterial {
        base: StandardMaterial {
            base_color_texture: Some(asset_server.load("aluminum/albedo.png")),
            metallic: 1.0,
            perceptual_roughness: 0.79,
            normal_map_texture: Some(asset_server.load("aluminum/normal.png")),
            ..Default::default()
        },
        extension: BuildingMaterial::default(),
    });

    building_material_handle.0 = material_handle;
}

#[derive(Component)]
struct Building;

fn toggle_build_mode(
    selection_query: Query<&SelectionSource, With<ActiveCamera>>,
    parent_query: Query<&Parent, With<ChunkPos>>,
    building_query: Query<(), With<Building>>,
    mut commands: Commands,
) {
    let Ok(selection_source) = selection_query.get_single() else {
        return;
    };

    let Some((intersected_entity, _)) = selection_source.intersection() else {
        return;
    };

    let grid_entity = match parent_query.get(intersected_entity) {
        Ok(parent) => parent.get(),
        Err(_) => {
            return;
        }
    };

    if let Ok(_) = building_query.get(grid_entity) {
        commands.entity(grid_entity).remove::<Building>();
    } else {
        commands.entity(grid_entity).insert(Building);
    }
}

fn update_grid_materials(
    building_grid_query: Query<&Children, Added<Building>>,
    chunk_query: Query<(), With<ChunkPos>>,
    mut not_building_grids: RemovedComponents<Building>,
    children_query: Query<&Children>,
    mut commands: Commands,
    grid_material_handle: Res<GridMaterialHandle>,
    building_material_handle: Res<BuildingMaterialHandle>,
) {
    for children in building_grid_query.iter() {
        for &child in children.iter() {
            if let Ok(_) = chunk_query.get(child) {
                commands
                    .entity(child)
                    .remove::<Handle<StandardMaterial>>()
                    .insert(building_material_handle.0.clone());
            }
        }
    }

    for children in children_query.iter_many(not_building_grids.read()) {
        for &child in children.iter() {
            if let Ok(_) = chunk_query.get(child) {
                commands
                    .entity(child)
                    .remove::<Handle<BuildingMaterialType>>()
                    .insert(grid_material_handle.0.clone());
            }
        }
    }
}

pub struct BuildingMaterialPlugin;

impl Plugin for BuildingMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<BuildingMaterialType>::default())
            .insert_resource(BuildingMaterialHandle(Handle::default()))
            .add_systems(Startup, init_building_material.in_set(AssetInitialization))
            .add_systems(
                Update,
                (
                    toggle_build_mode.run_if(input_just_pressed(KeyCode::B)),
                    update_grid_materials,
                )
                    .chain(),
            );
    }
}
