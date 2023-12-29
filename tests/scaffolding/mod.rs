use std::time::Duration;

use bevy::diagnostic::DiagnosticsPlugin;
use bevy::gizmos::GizmoPlugin;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::{ButtonState, InputPlugin};
use bevy::log::{Level, LogPlugin};
use bevy::pbr::PbrPlugin;
use bevy::prelude::*;
use bevy::render::settings::RenderCreation;
use bevy::render::{settings::WgpuSettings, RenderPlugin};
use bevy::scene::ScenePlugin;

use big_space::{FloatingOrigin, FloatingOriginPlugin};
use space_game::app_setup::{SetupGame, SetupMaterials};
use space_game::fixed_update::{FixedUpdateSet, SetupFixedTimeStepSchedule, SetupRapier};
use space_game::{UniverseGrid, UniverseGridPrecision, PHYSICS_TIMESTEP};

pub trait SetupBevyPlugins {
    fn setup_bevy_plugins(&mut self) -> &mut Self;
}

impl SetupBevyPlugins for App {
    fn setup_bevy_plugins(&mut self) -> &mut Self {
        self.add_plugins((
            MinimalPlugins,
            HierarchyPlugin,
            DiagnosticsPlugin,
            AssetPlugin::default(),
            ScenePlugin,
            RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    backends: None,
                    ..Default::default()
                }),
            },
            ImagePlugin::default(),
            LogPlugin {
                level: Level::ERROR,
                filter: "wgpu=error,naga=error".to_string(),
            },
            InputPlugin,
            WindowPlugin {
                primary_window: None,
                exit_condition: bevy::window::ExitCondition::DontExit,
                ..Default::default()
            },
            PbrPlugin::default(),
            FloatingOriginPlugin::<UniverseGridPrecision>::default(),
            GizmoPlugin,
        ))
    }
}

pub trait GameTest {
    fn game_test() -> Self;
}

impl GameTest for App {
    fn game_test() -> Self {
        let mut app = Self::new();

        app.setup_bevy_plugins()
            .setup_fixed_timestep_schedule()
            .setup_rapier()
            .setup_game()
            .setup_materials();

        // Spawn an entity to act as the floating origin so that the game doesn't crash
        app.world.spawn((
            TransformBundle::default(),
            UniverseGrid::default(),
            FloatingOrigin,
        ));

        app
    }
}

pub trait FixedUpdate {
    fn fixed_update(&mut self);
}

impl FixedUpdate for App {
    fn fixed_update(&mut self) {
        let overstep = self.world.get_resource::<Time<Fixed>>().unwrap().overstep();
        let mut time = self.world.get_resource_mut::<Time<Real>>().unwrap();

        if overstep <= Duration::from_secs_f32(PHYSICS_TIMESTEP) {
            time.advance_by(Duration::from_secs_f32(PHYSICS_TIMESTEP + 0.015) - overstep);
        }

        self.update();
    }
}

pub trait MockInput {
    fn mock_key_press(&mut self, key: KeyCode);
    fn mock_key_release(&mut self, key: KeyCode);
    fn mock_mouse_button_press(&mut self, button: MouseButton);
    fn mock_mouse_button_release(&mut self, button: MouseButton);
}

impl MockInput for App {
    fn mock_key_press(&mut self, key: KeyCode) {
        self.world
            .get_resource_mut::<Events<KeyboardInput>>()
            .unwrap()
            .send(KeyboardInput {
                scan_code: 0,
                key_code: Some(key),
                state: ButtonState::Pressed,
                window: Entity::PLACEHOLDER,
            });
    }

    fn mock_key_release(&mut self, key: KeyCode) {
        self.world
            .get_resource_mut::<Events<KeyboardInput>>()
            .unwrap()
            .send(KeyboardInput {
                scan_code: 0,
                key_code: Some(key),
                state: ButtonState::Released,
                window: Entity::PLACEHOLDER,
            });
    }

    fn mock_mouse_button_press(&mut self, button: MouseButton) {
        self.world
            .get_resource_mut::<Events<MouseButtonInput>>()
            .unwrap()
            .send(MouseButtonInput {
                button,
                state: ButtonState::Pressed,
                window: Entity::PLACEHOLDER,
            });
    }

    fn mock_mouse_button_release(&mut self, button: MouseButton) {
        self.world
            .get_resource_mut::<Events<MouseButtonInput>>()
            .unwrap()
            .send(MouseButtonInput {
                button,
                state: ButtonState::Released,
                window: Entity::PLACEHOLDER,
            });
    }
}

#[test]
fn fixed_update_works() {
    let mut app = App::game_test();

    #[derive(Resource)]
    struct TestResource(pub u32);

    fn test_me(mut test_resource: ResMut<TestResource>) {
        test_resource.0 += 1;
    }

    app.insert_resource(TestResource(0));
    app.add_systems(FixedUpdate, test_me.in_set(FixedUpdateSet::Update));

    app.fixed_update();

    assert_eq!(1, app.world.get_resource::<TestResource>().unwrap().0);

    app.fixed_update();

    assert_eq!(2, app.world.get_resource::<TestResource>().unwrap().0);
}
