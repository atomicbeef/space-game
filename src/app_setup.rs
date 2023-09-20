use bevy::prelude::*;
use bevy::log::{Level, LogPlugin};
use bevy::window::close_on_esc;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::render::RapierDebugRenderPlugin;

use crate::camera::{CameraPlugin, CameraDebugPlugin};
use crate::free_camera::FreeCameraPlugin;
use crate::player::PlayerPlugin;
use crate::player_controller::PlayerControllerPlugin;
use crate::settings::Settings;

pub trait SetupBevyPlugins {
    fn setup_bevy_plugins(&mut self) -> &mut Self;
}

impl SetupBevyPlugins for App {
    fn setup_bevy_plugins(&mut self) -> &mut Self {
        self.add_plugins(DefaultPlugins.set(LogPlugin {
            level: Level::DEBUG,
            filter: "wgpu=error,naga=error".to_string()
        }))
    }
}

pub trait SetupGame {
    fn setup_game(&mut self) -> &mut Self;
}

impl SetupGame for App {
    fn setup_game(&mut self) -> &mut Self {
        self.insert_resource(Settings::default());
        self.add_systems(Update, close_on_esc);
        self.add_plugins((
            CameraPlugin,
            FreeCameraPlugin,
            PlayerPlugin,
            PlayerControllerPlugin,
        ))
    }
}

pub trait SetupDebug {
    fn setup_debug(&mut self) -> &mut Self;
}

impl SetupDebug for App {
    fn setup_debug(&mut self) -> &mut Self {
        self.add_plugins((
            RapierDebugRenderPlugin::default(),
            WorldInspectorPlugin::new(),
            CameraDebugPlugin,
        ))
    }
}