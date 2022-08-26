use bevy::prelude::*;

use crate::{
    c_client::{
        Client,
        DockerClient, // , DummyClient
    },
    c_tank::Tank,
    CCollider, CollisionType, TickState,
};
use bevy_rapier2d::prelude::*;

use crate::{c_command::CommandSource, c_event::EventSink, c_health::Health, collision_mask};

pub fn setup_sim_tanks(state: Res<TickState>, mut commands: Commands) {
    for (i, tank_id) in state.tank_ids.iter().enumerate() {
        commands
            .spawn()
            .insert(CCollider {
                collision_type: CollisionType::Tank,
            })
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Sleeping::disabled())
            .insert(Ccd::enabled())
            .insert(Tank { cooldown: 0 })
            .insert(Health {
                val: Health::MAX_HEALTH,
            })
            .insert(CommandSource::default())
            .insert(EventSink::default())
            .insert(GravityScale(0.0))
            .insert(RigidBody::Dynamic)
            .insert(ColliderMassProperties::Mass(1.0))
            .insert(ColliderMassProperties::Density(1.0))
            .insert(Collider::cuboid(19.0, 23.0))
            .insert(Restitution::coefficient(0.1))
            .insert(CollisionGroups::new(
                collision_mask::TANK,
                collision_mask::ALL,
            ))
            .insert(Damping {
                linear_damping: 0.5,
                angular_damping: 0.5,
            })
            .insert(Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            })
            .insert(Client {
                client: Box::new(DockerClient {
                    tank_id: tank_id.to_string(),
                }),
            })
            .insert_bundle(SpatialBundle {
                transform: Transform::from_xyz(150.0 * (i as f32) + 10.0, 0.0, 0.0),
                // transform: Transform::from_xyz(10.0, 20.0, 30.0),
                visibility: Visibility { is_visible: true },
                ..default()
            });
    }
}
