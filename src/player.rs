use bevy::{prelude::*, ecs::system::{Command, SystemState}};
use bevy_rapier3d::prelude::*;

use crate::player_camera::{PlayerCameraBundle, PlayerCamera};
use crate::fixed_update::FixedUpdateSet;
use crate::camera::ActiveCamera;
use crate::player_controller::ActivelyControlled;
use crate::fixed_update::AddFixedEvent;

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
}

#[derive(Event)]
pub struct PlayerSpawned(pub Entity);

pub struct SpawnPlayer {
    pub transform: Transform,
}

impl Command for SpawnPlayer {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(
            ResMut<Assets<Mesh>>,
            ResMut<Assets<StandardMaterial>>,
            Commands,
            EventWriter<PlayerSpawned>,
        )> = SystemState::new(world);

        let (
            mut meshes,
            mut materials,
            mut commands,
            mut spawn_events,
        ) = system_state.get_mut(world);

        let id = commands.spawn(PlayerBundle {
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
                0.5
            ),
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::empty(),
            damping: Damping { linear_damping: 2.0, angular_damping: 4.0 },
            external_impulse: ExternalImpulse::default(),
        }).with_children(|parent| {
            parent.spawn(PlayerCameraBundle::new(Transform::from_xyz(0.0, 0.95, 0.0)));
        }).id();

        spawn_events.send(PlayerSpawned(id));

        system_state.apply(world);
    }
}

impl SpawnPlayer {
    pub fn new(transform: Transform) -> Self {
        Self { transform }
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
    for event in spawned_events.iter() {
        // Remove ActivelyControlled component from old entity
        if let Ok(old_actively_controlled) = actively_controlled_query.get_single() {
            commands.entity(old_actively_controlled).remove::<ActivelyControlled>();
        }

        // Remove ActiveCamera component from old camera
        if let Ok((old_entity, mut old_camera)) = camera_set.p0().get_single_mut() {
            commands.entity(old_entity).remove::<ActiveCamera>();
            old_camera.is_active = false;
        }

        // Set new player to actively controlled
        commands.entity(event.0).insert(ActivelyControlled);

        // Set player camera to active
        let Ok(new_player_children) = children_query.get(event.0) else { return; };
        for &child in new_player_children {
            if let Ok(mut camera) = camera_set.p1().get_mut(child) {
                commands.entity(child).insert(ActiveCamera);
                camera.is_active = true;
                break;
            }
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_fixed_event::<PlayerSpawned>()
            .add_systems(FixedUpdate, control_newly_spawned_player.in_set(FixedUpdateSet::Update));
    }
}