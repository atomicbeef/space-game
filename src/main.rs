use bevy::pbr::ExtendedMaterial;
use bevy::prelude::*;

use bevy::render::view::RenderLayers;
use big_space::FloatingOrigin;
use space_game::UniverseGrid;
use space_game::app_setup::{SetupGame, SetupBevyPlugins, SetupDebug, SetupMaterials};
use space_game::block::{Block, BlockMaterial};
use space_game::building_material::BuildingMaterial;
use space_game::camera::ActiveCamera;
use space_game::fixed_update::{SetupFixedTimeStepSchedule, SetupRapier};
use space_game::free_camera::FreeCamera;
use space_game::grid::{command::SpawnGrid, Grid, GridPos};
use space_game::player::SpawnPlayer;

fn main() {
    App::new()
        .setup_bevy_plugins()
        .setup_fixed_timestep_schedule()
        .setup_rapier()
        .setup_game()
        .setup_materials()
        .setup_debug()
        .add_systems(Startup, setup_test_scene)
        .run();
}

fn setup_test_scene(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut building_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, BuildingMaterial>>>,
    mut commands: Commands,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        FreeCamera,
        ActiveCamera,
        FloatingOrigin,
        // Show the locally controlled player in the free camera
        RenderLayers::from_layers(&[0, 1]),
        UniverseGrid::default(),
    ));

    commands.add(SpawnPlayer::new(Transform::from_xyz(50000.0, 0.0, 40.0), UniverseGrid::default()));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.5 })),
            material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
            transform: Transform::from_xyz(50005.0, 0.0, 0.0),
            ..default()
        },
        UniverseGrid::default(),
    ));

    let mut cube_grid = Grid::new();
    for x in -5..5 {
        for y in -5..5 {
            for z in -5..5 {
                cube_grid.set(GridPos::new(x, y, z), Block { material: BlockMaterial::Aluminum });
            }
        }
    }

    commands.add(SpawnGrid::new(Transform::from_xyz(50000.0, 0.0, 0.0), cube_grid));

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

    commands.add(SpawnPlayer::new(Transform::from_xyz(0.0, 0.0, 20.0), UniverseGrid::default()));
}