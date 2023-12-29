use bevy::{
    ecs::system::{Command, SystemState},
    prelude::*,
    transform::TransformSystem,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_rapier3d::prelude::*;
use big_space::FloatingOrigin;

use crate::camera::ActiveCamera;
use crate::fixed_update::FixedUpdateSet;
use crate::player_controller::ActivelyControlled;
use crate::{
    player_camera::{PlayerCamera, PlayerCameraBundle},
    UniverseGrid,
};

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub pbr: PbrBundle,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub locked_axes: LockedAxes,
    pub damping: Damping,
    pub external_impulse: ExternalImpulse,
    pub grid_cell: UniverseGrid,
}

#[derive(Event)]
pub struct PlayerSpawned(pub Entity);

pub struct SpawnPlayer {
    pub transform: Transform,
    pub grid_cell: UniverseGrid,
}

impl Command for SpawnPlayer {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(
            ResMut<Assets<Mesh>>,
            ResMut<Assets<StandardMaterial>>,
            Commands,
            EventWriter<PlayerSpawned>,
        )> = SystemState::new(world);

        let (mut meshes, mut materials, mut commands, mut spawn_events) =
            system_state.get_mut(world);

        let id = commands
            .spawn(PlayerBundle {
                player: Player,
                pbr: PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Capsule {
                        radius: 0.5,
                        depth: 2.0,
                        ..Default::default()
                    })),
                    material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
                    transform: self.transform,
                    ..Default::default()
                },
                collider: Collider::capsule(
                    Vec3::new(0.0, 1.0, 0.0),
                    Vec3::new(0.0, -1.0, 0.0),
                    0.5,
                ),
                rigid_body: RigidBody::Dynamic,
                locked_axes: LockedAxes::empty(),
                damping: Damping {
                    linear_damping: 2.0,
                    angular_damping: 4.0,
                },
                external_impulse: ExternalImpulse::default(),
                grid_cell: self.grid_cell,
            })
            .with_children(|parent| {
                parent.spawn(PlayerCameraBundle::new(Transform::from_xyz(0.0, 0.95, 0.0)));
            })
            .id();

        spawn_events.send(PlayerSpawned(id));

        system_state.apply(world);
    }
}

impl SpawnPlayer {
    pub fn new(transform: Transform, grid_cell: UniverseGrid) -> Self {
        Self {
            transform,
            grid_cell,
        }
    }
}

fn control_newly_spawned_player(
    mut spawned_events: EventReader<PlayerSpawned>,
    mut commands: Commands,
    actively_controlled_query: Query<Entity, With<ActivelyControlled>>,
    mut camera_set: ParamSet<(
        Query<(Entity, &mut Camera), With<ActiveCamera>>,
        Query<&mut Camera, With<PlayerCamera>>,
    )>,
    children_query: Query<&Children>,
) {
    if let Some(event) = spawned_events.read().last() {
        // Remove ActivelyControlled component from old entity
        if let Ok(old_actively_controlled) = actively_controlled_query.get_single() {
            commands
                .entity(old_actively_controlled)
                .remove::<(ActivelyControlled, FloatingOrigin)>();
        }

        // Remove ActiveCamera component from old camera
        if let Ok((old_entity, mut old_camera)) = camera_set.p0().get_single_mut() {
            commands
                .entity(old_entity)
                .remove::<(ActiveCamera, FloatingOrigin)>();
            old_camera.is_active = false;
        }

        // Set new player to actively controlled
        commands
            .entity(event.0)
            .insert((ActivelyControlled, FloatingOrigin));

        // Set player camera to active
        let Ok(new_player_children) = children_query.get(event.0) else {
            return;
        };
        for &child in new_player_children {
            if let Ok(mut camera) = camera_set.p1().get_mut(child) {
                commands.entity(child).insert(ActiveCamera);
                camera.is_active = true;
                break;
            }
        }
    }
}

fn draw_reticle(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_auery: Query<&GlobalTransform, (With<PlayerCamera>, With<ActiveCamera>)>,
    mut gizmos: Gizmos,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };

    if matches!(window.cursor.grab_mode, CursorGrabMode::None) {
        return;
    }

    let Ok(camera_transform) = camera_auery.get_single() else {
        return;
    };

    gizmos.circle(
        camera_transform.translation() + camera_transform.forward() * 3.0,
        -camera_transform.forward(),
        0.005,
        Color::WHITE,
    );
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerSpawned>()
            .add_systems(
                PostUpdate,
                draw_reticle.after(TransformSystem::TransformPropagate),
            )
            .add_systems(
                FixedUpdate,
                control_newly_spawned_player.in_set(FixedUpdateSet::Update),
            );
    }
}
