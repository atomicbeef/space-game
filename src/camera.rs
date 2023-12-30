use bevy::prelude::*;
use big_space::FloatingOrigin;

use crate::player_controller::ActivelyControlled;
use crate::settings::Settings;

#[derive(Component)]
pub struct ActiveCamera;

fn cycle_cameras(
    input: Res<Input<KeyCode>>,
    mut active_camera_query: Query<(Entity, Option<&Parent>), With<ActiveCamera>>,
    mut camera_query: Query<(Entity, Option<&Parent>, &mut Camera), Without<Camera2d>>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::F1) {
        if camera_query.iter().count() < 2 {
            return;
        }

        if let Ok((active_camera, potential_player)) = active_camera_query.get_single_mut() {
            // Deactivate old active camera
            commands
                .entity(active_camera)
                .remove::<(ActiveCamera, FloatingOrigin)>();
            if let Ok((_, _, mut camera)) = camera_query.get_mut(active_camera) {
                camera.is_active = false;
            }

            if let Some(player) = potential_player {
                commands
                    .entity(player.get())
                    .remove::<(ActivelyControlled, FloatingOrigin)>();
            }

            let mut camera_data: Vec<(Entity, Option<&Parent>, Mut<'_, Camera>)> =
                camera_query.iter_mut().collect();
            camera_data.sort_by(|(entity, _, _), (other_entity, _, _)| entity.cmp(other_entity));

            let current_camera_index = camera_data
                .iter()
                .position(|(entity, _, _)| *entity == active_camera)
                .unwrap();
            let (new_active_camera, new_potential_player, camera) =
                if current_camera_index == camera_data.len() - 1 {
                    &mut camera_data[0]
                } else {
                    &mut camera_data[current_camera_index + 1]
                };

            commands.entity(*new_active_camera).insert(ActiveCamera);
            camera.is_active = true;

            match new_potential_player {
                Some(player) => {
                    commands
                        .entity(player.get())
                        .insert((ActivelyControlled, FloatingOrigin));
                }
                None => {
                    commands.entity(*new_active_camera).insert(FloatingOrigin);
                }
            };
        }
    }
}

fn draw_cameras(
    camera_transform_query: Query<&GlobalTransform, (With<Camera>, Without<ActiveCamera>)>,
    mut gizmos: Gizmos,
    settings: Res<Settings>,
) {
    if !settings.draw_debug {
        return;
    }

    for camera_transform in camera_transform_query.iter() {
        gizmos.sphere(
            camera_transform.translation(),
            Quat::IDENTITY,
            0.2,
            Color::GREEN,
        );
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (cycle_cameras,));
    }
}

pub struct CameraDebugPlugin;

impl Plugin for CameraDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (draw_cameras,));
    }
}
