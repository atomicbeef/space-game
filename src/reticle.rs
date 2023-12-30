use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use crate::{camera::ActiveCamera, player_camera::PlayerCamera};

#[derive(Component)]
pub struct Reticle;

fn update_reticle_visibility(
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_camera_query: Query<(), (With<PlayerCamera>, With<ActiveCamera>)>,
    mut reticle_query: Query<&mut Visibility, With<Reticle>>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };

    let Ok(mut visibility) = reticle_query.get_single_mut() else {
        return;
    };

    if player_camera_query.iter().count() > 0 && window.cursor.grab_mode != CursorGrabMode::None {
        *visibility = Visibility::Visible;
    } else {
        *visibility = Visibility::Hidden;
    }
}

pub struct ReticlePlugin;

impl Plugin for ReticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_reticle_visibility);
    }
}
