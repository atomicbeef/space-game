use bevy::prelude::*;
use bevy_rapier3d::render::DebugRenderContext;

#[derive(Resource)]
pub struct Settings {
    pub first_person_sensitivity: f32,
    pub free_camera_sensitivity: f32,
    pub camera_speed: f32,
    pub fullscreen: bool,
    pub draw_debug: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            first_person_sensitivity: 0.00025,
            free_camera_sensitivity: 0.00025,
            camera_speed: 12.5,
            fullscreen: false,
            draw_debug: true,
        }
    }
}

fn toggle_debug_draw(
    input: Res<Input<KeyCode>>,
    mut settings: ResMut<Settings>,
    mut rapier_debug_render_context: ResMut<DebugRenderContext>,
) {
    if input.just_pressed(KeyCode::F3) {
        settings.draw_debug = !settings.draw_debug;
        rapier_debug_render_context.enabled = settings.draw_debug;
    }
}

pub struct DebugSettingsPlugin;

impl Plugin for DebugSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_debug_draw);
    }
}