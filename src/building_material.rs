use bevy::prelude::*;
use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::render::render_resource::{ShaderRef, AsBindGroup};

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

pub struct BuildingMaterialPlugin;

impl Plugin for BuildingMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            MaterialPlugin::<ExtendedMaterial<StandardMaterial, BuildingMaterial>>::default()
        );
    }
}