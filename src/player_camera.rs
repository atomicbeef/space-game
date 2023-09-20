use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Bundle)]
pub struct PlayerCameraBundle {
    pub camera: Camera3dBundle,
    pub player_camera: PlayerCamera,
}

impl PlayerCameraBundle {
    pub fn new(transform: Transform) -> Self {
        Self {
            camera: Camera3dBundle {
                transform,
                camera: Camera { is_active: false, ..Default::default() },
                ..Default::default()
            },
            player_camera: PlayerCamera,
        }
    }
}