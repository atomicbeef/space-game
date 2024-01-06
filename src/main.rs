use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::pbr::ExtendedMaterial;
use bevy::prelude::*;

use bevy::render::view::RenderLayers;
use bevy::sprite::MaterialMesh2dBundle;
use big_space::FloatingOrigin;
use space_game::app_setup::{
    AssetInitialization, SetupBevyPlugins, SetupDebug, SetupGame, SetupMaterials,
};
use space_game::building::BuildMarker;
use space_game::building_material::BuildingMaterial;
use space_game::camera::ActiveCamera;
use space_game::fixed_update::{SetupFixedTimeStepSchedule, SetupRapier};
use space_game::free_camera::FreeCamera;
use space_game::grid::block::{Block, BlockMaterial, BLOCK_SIZE};
use space_game::grid::chunk::{Chunk, CHUNK_SIZE_CUBED};
use space_game::grid::{command::SpawnGrid, ChunkPos, Grid};
use space_game::player::SpawnPlayer;
use space_game::raycast_selection::SelectionSource;
use space_game::reticle::Reticle;
use space_game::UniverseGrid;

fn main() {
    App::new()
        .setup_bevy_plugins()
        .setup_fixed_timestep_schedule()
        .setup_rapier()
        .setup_game()
        .setup_materials()
        .setup_debug()
        .add_systems(Startup, setup_test_scene.after(AssetInitialization))
        .run();
}

fn setup_test_scene(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut building_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, BuildingMaterial>>>,
    mut commands: Commands,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        FreeCamera,
        ActiveCamera,
        SelectionSource::new(),
        FloatingOrigin,
        // Show the locally controlled player in the free camera
        RenderLayers::from_layers(&[0, 1]),
        UniverseGrid::default(),
    ));

    commands.add(SpawnPlayer::new(
        Transform::from_xyz(50000.0, 0.0, 40.0),
        UniverseGrid::default(),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.5 })),
            material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
            transform: Transform::from_xyz(50005.0, 0.0, 0.0),
            ..default()
        },
        UniverseGrid::default(),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: BLOCK_SIZE })),
            material: materials.add(Color::rgba(0.0, 0.0, 1.0, 0.5).into()),
            ..default()
        },
        UniverseGrid::default(),
        BuildMarker,
    ));

    let mut cube_grid = Grid::new();
    let chunk = Chunk::new(
        Entity::PLACEHOLDER,
        [Block {
            material: BlockMaterial::Aluminum,
        }; CHUNK_SIZE_CUBED],
    );
    cube_grid.set_chunk(ChunkPos::new(0, 0, 0), Some(chunk));

    commands.add(SpawnGrid::new(
        Transform::from_xyz(50000.0, 0.0, 0.0),
        cube_grid,
    ));

    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.5 })),
            material: building_materials.add(ExtendedMaterial {
                base: Color::rgb(1.0, 0.0, 0.0).into(),
                extension: BuildingMaterial::default(),
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        UniverseGrid::default(),
    ));

    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 1,
            ..Default::default()
        },
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::None,
        },
        ..Default::default()
    });

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::new(2.5))).into(),
            material: color_materials.add(ColorMaterial::from(Color::WHITE)),
            ..Default::default()
        },
        Reticle,
    ));

    commands.add(SpawnPlayer::new(
        Transform::from_xyz(0.0, 0.0, 20.0),
        UniverseGrid::default(),
    ));
}
