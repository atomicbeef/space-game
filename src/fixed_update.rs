use std::hash::Hash;
use std::marker::PhantomData;

use bevy::input::InputSystem;
use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy_rapier3d::prelude::*;

use crate::{UniverseGridPrecision, PHYSICS_TIMESTEP};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum FixedUpdateSet {
    PreUpdate,
    PreUpdateFlush,
    Update,
    UpdateFlush,
    PostUpdate,
    PostUpdateFlush,
    Last,
    LastFlush,
}

// A set for `propagate_transforms` to mark it as ambiguous with `sync_simple_transforms`.
// Used instead of the `SystemTypeSet` as that would not allow multiple instances of the system.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
struct PropagateTransformsSet;

pub trait SetupFixedTimeStepSchedule {
    fn setup_fixed_timestep_schedule(&mut self) -> &mut Self;
}

impl SetupFixedTimeStepSchedule for App {
    fn setup_fixed_timestep_schedule(&mut self) -> &mut Self {
        self.edit_schedule(FixedUpdate, |schedule| {
            schedule.configure_sets(
                (
                    FixedUpdateSet::PreUpdate,
                    FixedUpdateSet::PreUpdateFlush,
                    FixedUpdateSet::Update,
                    FixedUpdateSet::UpdateFlush,
                    PhysicsSet::SyncBackend,
                    PhysicsSet::StepSimulation,
                    PhysicsSet::Writeback,
                    FixedUpdateSet::PostUpdate,
                    FixedUpdateSet::PostUpdateFlush,
                    FixedUpdateSet::Last,
                    FixedUpdateSet::LastFlush,
                )
                    .chain(),
            );

            schedule.configure_sets(
                TransformSystem::TransformPropagate.in_set(FixedUpdateSet::PostUpdate),
            );
            schedule
                .configure_sets(PropagateTransformsSet.in_set(TransformSystem::TransformPropagate));

            schedule.add_systems(apply_deferred.in_set(FixedUpdateSet::PreUpdateFlush));
            schedule.add_systems(apply_deferred.in_set(FixedUpdateSet::UpdateFlush));
            schedule.add_systems(apply_deferred.in_set(FixedUpdateSet::PostUpdateFlush));
            schedule.add_systems(apply_deferred.in_set(FixedUpdateSet::LastFlush));
        });

        self.add_plugins(FixedInputPlugin)
    }
}

pub trait SetupRapier {
    fn setup_rapier(&mut self) -> &mut Self;
}

impl SetupRapier for App {
    fn setup_rapier(&mut self) -> &mut Self {
        let rapier_config = RapierConfiguration {
            timestep_mode: TimestepMode::Interpolated {
                dt: PHYSICS_TIMESTEP,
                substeps: 1,
                time_scale: 1.0,
            },
            gravity: Vec3::default(),
            force_update_from_transform_changes: true,
            ..Default::default()
        };

        self.insert_resource(rapier_config).add_plugins(
            RapierPhysicsPlugin::<NoUserData, UniverseGridPrecision>::default().in_fixed_schedule(),
        )
    }
}

#[derive(Resource)]
pub struct Flag<T> {
    pub enabled: bool,
    _marker: PhantomData<T>,
}

impl<T> Default for Flag<T> {
    fn default() -> Self {
        Self {
            enabled: false,
            _marker: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Resource, Reflect, Deref, DerefMut)]
pub struct FixedInput<T: Copy + Eq + Hash + Send + Sync + 'static>(Input<T>);

impl<T: Copy + Eq + Hash + Send + Sync + 'static> Default for FixedInput<T> {
    fn default() -> Self {
        Self(Input::default())
    }
}

fn update_fixed_input<T: Copy + Eq + Hash + Send + Sync + 'static>(
    mut fixed_input: ResMut<FixedInput<T>>,
    input: Res<Input<T>>,
) {
    for pressed in input.get_just_pressed() {
        fixed_input.press(*pressed);
    }

    for released in input.get_just_released() {
        fixed_input.release(*released);
    }
}

fn set_clear_fixed_input<T: Copy + Eq + Hash + Send + Sync + 'static>(
    mut fixed_input: ResMut<FixedInput<T>>,
    mut flag: ResMut<Flag<Input<T>>>,
) {
    if flag.enabled {
        fixed_input.clear();
    }

    flag.enabled = true;
}

fn clear_fixed_input<T: Copy + Eq + Hash + Send + Sync + 'static>(
    mut fixed_input: ResMut<FixedInput<T>>,
    mut flag: ResMut<Flag<Input<T>>>,
) {
    if flag.enabled {
        fixed_input.clear();
    }

    flag.enabled = false;
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct FixedInputSystem;

fn add_fixed_input<T: Copy + Eq + Hash + Send + Sync + 'static>(app: &mut App) {
    app.init_resource::<Flag<Input<T>>>()
        .init_resource::<FixedInput<T>>()
        .add_systems(PreUpdate, update_fixed_input::<T>.after(InputSystem))
        .add_systems(
            FixedUpdate,
            set_clear_fixed_input::<T>.in_set(FixedInputSystem),
        )
        .add_systems(First, clear_fixed_input::<T>);
}

pub struct FixedInputPlugin;

impl Plugin for FixedInputPlugin {
    fn build(&self, app: &mut App) {
        add_fixed_input::<KeyCode>(app);
        add_fixed_input::<ScanCode>(app);
        add_fixed_input::<MouseButton>(app);
        add_fixed_input::<GamepadButton>(app);
    }
}
