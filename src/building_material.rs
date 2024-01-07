use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

use crate::app_setup::AssetInitialization;

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

#[derive(Resource)]
pub struct BuildingMaterialHandle(pub Handle<ExtendedMaterial<StandardMaterial, BuildingMaterial>>);

fn init_building_material(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, BuildingMaterial>>>,
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

pub struct BuildingMaterialPlugin;

impl Plugin for BuildingMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, BuildingMaterial>,
        >::default())
            .insert_resource(BuildingMaterialHandle(Handle::default()))
            .add_systems(Startup, init_building_material.in_set(AssetInitialization));
    }
}
