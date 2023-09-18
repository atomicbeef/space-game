use bevy::prelude::*;

use bevy::render::view::RenderLayers;
use space_game::app_setup::{SetupGame, SetupBevyPlugins, SetupDebug};
use space_game::camera::ActiveCamera;
use space_game::fixed_update::{SetupFixedTimeStepSchedule, SetupRapier};
use space_game::free_camera::FreeCamera;

fn main() {
    App::new()
        .setup_bevy_plugins()
        .setup_fixed_timestep_schedule()
        .setup_rapier()
        .setup_game()
        .setup_debug()
        .add_systems(Startup, setup_test_scene)
        .run();
}

fn setup_test_scene(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                is_active: true,
                ..Default::default()
            },
            ..Default::default()
        },
        ActiveCamera,
        FreeCamera,
        // Show the locally controlled player in the free camera
        RenderLayers::from_layers(&[0, 1]),
    ));

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}