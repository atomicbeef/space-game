use bevy::{input::common_conditions::input_just_pressed, prelude::*};

fn toggle_pause(mut time: ResMut<Time<Virtual>>) {
    if time.is_paused() {
        time.unpause();
    } else {
        time.pause();
    }
}

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_pause.run_if(input_just_pressed(KeyCode::F9)));
    }
}
