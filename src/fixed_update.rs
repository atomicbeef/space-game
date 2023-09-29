use std::hash::Hash;
use std::marker::PhantomData;

use bevy::prelude::*;
use bevy::input::{InputSystem, mouse::MouseMotion};
use bevy::transform::TransformSystem;
use bevy_rapier3d::prelude::*;
use big_space::propagation::propagate_transforms;
use big_space::{recenter_transform_on_grid, sync_simple_transforms, update_global_from_grid};

use crate::{PHYSICS_TIMESTEP, UniverseGridPrecision};

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
            schedule.configure_sets((
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
            ).chain());

            schedule.configure_set(TransformSystem::TransformPropagate.in_set(FixedUpdateSet::PostUpdate));
            schedule.configure_set(PropagateTransformsSet.in_set(TransformSystem::TransformPropagate));

            schedule.add_systems(apply_deferred.in_set(FixedUpdateSet::PreUpdateFlush));
            schedule.add_systems(apply_deferred.in_set(FixedUpdateSet::UpdateFlush));
            schedule.add_systems(apply_deferred.in_set(FixedUpdateSet::PostUpdateFlush));
            schedule.add_systems(apply_deferred.in_set(FixedUpdateSet::LastFlush));

            schedule.add_systems((
                recenter_transform_on_grid::<UniverseGridPrecision>,
                (sync_simple_transforms::<UniverseGridPrecision>, update_global_from_grid::<UniverseGridPrecision>)
                    .after(recenter_transform_on_grid::<UniverseGridPrecision>)
                    .before(propagate_transforms::<UniverseGridPrecision>),
                propagate_transforms::<UniverseGridPrecision>,
            ).in_set(TransformSystem::TransformPropagate));
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
            timestep_mode: TimestepMode::Fixed {
                dt: PHYSICS_TIMESTEP,
                substeps: 1,
            },
            gravity: Vec3::default(),
            ..Default::default()
        };
    
        self.insert_resource(rapier_config)
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default().in_fixed_schedule())
            
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

fn set_update_events<T: Event>(mut update_events_flag: ResMut<Flag<Events<T>>>) {
    update_events_flag.enabled = true;
}

fn update_events<T: Event>(mut update_events_flag: ResMut<Flag<Events<T>>>, mut events: ResMut<Events<T>>) {
    if update_events_flag.enabled {
        events.update();
        update_events_flag.enabled = false;
    }
}

pub trait AddFixedEvent {
    fn add_fixed_event<T: Event>(&mut self) -> &mut Self;
}

impl AddFixedEvent for App {
    fn add_fixed_event<T: Event>(&mut self) -> &mut Self {
        self.init_resource::<Flag<Events<T>>>()
            .init_resource::<Events<T>>()
            .add_systems(FixedUpdate, (
                set_update_events::<T>.in_set(FixedUpdateSet::PreUpdate),
                update_events::<T>.in_set(FixedUpdateSet::Last),
            ))
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
    input: Res<Input<T>>
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
    mut flag: ResMut<Flag<Input<T>>>
) {
    if flag.enabled {
        fixed_input.clear();
    }

    flag.enabled = true;
}

fn clear_fixed_input<T: Copy + Eq + Hash + Send + Sync + 'static>(
    mut fixed_input: ResMut<FixedInput<T>>,
    mut flag: ResMut<Flag<Input<T>>>
) {
    if flag.enabled {
        fixed_input.clear();
    }

    flag.enabled = false;
}

#[derive(Debug, Clone, Resource, Reflect, Deref, DerefMut, Event)]
pub struct FixedMouseMotion(MouseMotion);

fn send_fixed_mouse_motion_events(
    mut mouse_motion_reader: EventReader<MouseMotion>,
    mut fixed_mouse_motion_wrier: EventWriter<FixedMouseMotion>
) {
    for event in mouse_motion_reader.iter() {
        fixed_mouse_motion_wrier.send(FixedMouseMotion(*event));
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct FixedInputSystem;

fn add_fixed_input<T: Copy + Eq + Hash + Send + Sync + 'static>(app: &mut App) {
    app.init_resource::<Flag<Input<T>>>()
        .init_resource::<FixedInput<T>>()
        .add_systems(PreUpdate, update_fixed_input::<T>.after(InputSystem))
        .add_systems(FixedUpdate, set_clear_fixed_input::<T>.in_set(FixedInputSystem))
        .add_systems(First, clear_fixed_input::<T>);
}

pub struct FixedInputPlugin;

impl Plugin for FixedInputPlugin {
    fn build(&self, app: &mut App) {
        add_fixed_input::<KeyCode>(app);
        add_fixed_input::<ScanCode>(app);
        add_fixed_input::<MouseButton>(app);
        add_fixed_input::<GamepadButton>(app);
        app.add_fixed_event::<FixedMouseMotion>();
        app.add_systems(FixedUpdate, send_fixed_mouse_motion_events.in_set(FixedInputSystem));
    }
}