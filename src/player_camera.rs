use bevy::{core_pipeline::Skybox, prelude::*};

use crate::raycast_selection::SelectionSource;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Bundle)]
pub struct PlayerCameraBundle {
    pub camera: Camera3dBundle,
    pub player_camera: PlayerCamera,
    pub selection_source: SelectionSource,
    pub skybox: Skybox,
}

impl PlayerCameraBundle {
    pub fn new(transform: Transform, skybox: Skybox) -> Self {
        Self {
            camera: Camera3dBundle {
                transform,
                camera: Camera {
                    is_active: false,
                    ..Default::default()
                },
                ..Default::default()
            },
            player_camera: PlayerCamera,
            selection_source: SelectionSource::new(),
            skybox,
        }
    }
}
