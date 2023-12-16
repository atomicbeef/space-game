use big_space::GridCell;

pub mod app_setup;
pub mod building_material;
pub mod camera;
pub mod fixed_update;
pub mod free_camera;
pub mod grid;
pub mod player;
pub mod player_camera;
pub mod player_controller;
pub mod settings;

pub const PHYSICS_TIMESTEP: f32 = 1.0 / 64.0;
pub type UniverseGridPrecision = i32;
pub type UniverseGrid = GridCell<UniverseGridPrecision>;
