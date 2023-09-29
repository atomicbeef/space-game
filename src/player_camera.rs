use bevy::prelude::*;

use crate::UniverseGrid;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Bundle)]
pub struct PlayerCameraBundle {
    pub camera: Camera3dBundle,
    pub player_camera: PlayerCamera,
    pub grid_cell: UniverseGrid,
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
            grid_cell: UniverseGrid::default(),
        }
    }
}