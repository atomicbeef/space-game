use std::time::Duration;

use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::log::{LogPlugin, Level};
use bevy::diagnostic::DiagnosticsPlugin; 
use bevy::render::{RenderPlugin, settings::WgpuSettings};
use bevy::scene::ScenePlugin;
use bevy::input::{InputPlugin, ButtonState};
use bevy::pbr::PbrPlugin;

use big_space::{FloatingOriginPlugin, FloatingOrigin};
use space_game::app_setup::SetupGame;
use space_game::fixed_update::{SetupFixedTimeStepSchedule, SetupRapier, FixedUpdateSet};
use space_game::{PHYSICS_TIMESTEP, UniverseGrid, UniverseGridPrecision};

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
                wgpu_settings: WgpuSettings {
                    backends: None,
                    ..Default::default()
                }
            },
            ImagePlugin::default(),
            LogPlugin {
                level: Level::ERROR,
                filter: "wgpu=error,naga=error".to_string()
            },
            InputPlugin,
            WindowPlugin {
                primary_window: None,
                exit_condition: bevy::window::ExitCondition::DontExit,
                ..Default::default()
            },
            PbrPlugin::default(),
            FloatingOriginPlugin::<UniverseGridPrecision>::default()
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
            .setup_game();

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
        let mut time = self.world.get_resource_mut::<FixedTime>().unwrap();

        let accumulated = time.accumulated();
        if accumulated < Duration::from_secs_f32(PHYSICS_TIMESTEP) {
            time.tick(Duration::from_secs_f32(PHYSICS_TIMESTEP) - accumulated);
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
        self.world.get_resource_mut::<Events<KeyboardInput>>().unwrap().send(KeyboardInput {
            scan_code: 0,
            key_code: Some(key),
            state: ButtonState::Pressed,
            window: Entity::PLACEHOLDER,
        });
    }

    fn mock_key_release(&mut self, key: KeyCode) {
        self.world.get_resource_mut::<Events<KeyboardInput>>().unwrap().send(KeyboardInput {
            scan_code: 0,
            key_code: Some(key),
            state: ButtonState::Released,
            window: Entity::PLACEHOLDER,
        });
    }

    fn mock_mouse_button_press(&mut self, button: MouseButton) {
        self.world.get_resource_mut::<Events<MouseButtonInput>>().unwrap().send(MouseButtonInput {
            button,
            state: ButtonState::Pressed,
            window: Entity::PLACEHOLDER,
        });
    }

    fn mock_mouse_button_release(&mut self, button: MouseButton) {
        self.world.get_resource_mut::<Events<MouseButtonInput>>().unwrap().send(MouseButtonInput {
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
}