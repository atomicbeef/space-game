use bevy::ecs::event::EventReader;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

use crate::camera::ActiveCamera;
use crate::fixed_update::{FixedInput, FixedMouseMotion};
use crate::settings::Settings;
use crate::fixed_update::FixedUpdateSet;
use crate::PHYSICS_TIMESTEP;

#[derive(Component)]
pub struct FreeCamera;

fn camera_move(
    keys: Res<FixedInput<KeyCode>>,
    settings: Res<Settings>,
    mut query: Query<&mut Transform, (With<ActiveCamera>, With<FreeCamera>)>,
) {
    for mut transform in query.iter_mut() {
        let mut velocity = Vec3::ZERO;

        for key in keys.get_pressed() {
            match key {
                KeyCode::W => velocity += transform.forward(),
                KeyCode::S => velocity += transform.back(),
                KeyCode::D => velocity += transform.right(),
                KeyCode::A => velocity += transform.left(),
                KeyCode::Space => velocity += Vec3::Y,
                KeyCode::C => velocity -= Vec3::Y,
                _ => (),
            }
        }

        velocity = velocity.normalize_or_zero();

        transform.translation += velocity * settings.camera_speed * PHYSICS_TIMESTEP;
    }
}

fn camera_rotate(
    mut motion_evr: EventReader<FixedMouseMotion>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    settings: Res<Settings>,
    mut camera_query: Query<&mut Transform, (With<ActiveCamera>, With<FreeCamera>)>,
) {
    let primary_window = primary_window_query.get_single();
    if let Ok(window) = primary_window {
        for mut transform in camera_query.iter_mut() {
            for ev in motion_evr.iter() {
                match window.cursor.grab_mode {
                    CursorGrabMode::None => {},
                    CursorGrabMode::Confined | CursorGrabMode::Locked => {
                        let scale_factor = window.height().min(window.width());
                        let pitch = (settings.free_camera_sensitivity * ev.delta.y * scale_factor).to_radians();
                        let yaw = (settings.free_camera_sensitivity * ev.delta.x * scale_factor).to_radians();

                        transform.rotate_y(-yaw);
                        transform.rotate_local_x(-pitch);
                    }
                }
            }
        }
    }
}

fn cursor_grab(
    mouse_button_input: Res<FixedInput<MouseButton>>,
    mut primary_window_query: Query<&mut Window, With<PrimaryWindow>>,
    free_camera_query: Query<(), (With<ActiveCamera>, With<FreeCamera>)>
) {
    // Only manage the cursor if the active camera is a free camera
    let Ok(_) = free_camera_query.get_single() else {
        return;
    };

    let primary_window = primary_window_query.get_single_mut();
    if let Ok(mut window) = primary_window {
        // Lock and hide the cursor if RMB is pressed
        let rmb_pressed = mouse_button_input.pressed(MouseButton::Right);
        let cursor_locked = match window.cursor.grab_mode {
            CursorGrabMode::None => false,
            CursorGrabMode::Confined | CursorGrabMode::Locked => true
        };
        
        if rmb_pressed && !cursor_locked {
            window.cursor.grab_mode = CursorGrabMode::Confined;
            window.cursor.visible = false;
        } else if !rmb_pressed && cursor_locked {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}

pub struct FreeCameraPlugin;

impl Plugin for FreeCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (
            camera_move,
            camera_rotate,
            cursor_grab
        ).in_set(FixedUpdateSet::Update));
    }
}