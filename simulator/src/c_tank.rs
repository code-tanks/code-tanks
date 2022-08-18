use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{
    c_command::CommandSource, c_event::EventSink, c_health::Health, c_render::Render,
    c_scanner::Scanner,
};

#[derive(Component)]
pub struct Tank {}

pub fn create_tank(mut commands: Commands, n: usize) -> EntityCommands {
    commands
        .spawn()
        .insert(Render::as_tank())
        .insert(Health {})
        // .insert(Position {
        //     x: 0.0,
        //     y: 0.0,
        //     rotation: 0.0,
        // })
        // .insert(CVelocity { velocity: 0.0 })
        // .insert(TankVelocity {
        //     angular_velocity: 0.0,
        //     gun_angular_velocity: 0.0,
        //     radar_angular_velocity: 0.0,
        // })
        // .insert(CCollider::tank())
        .insert(CommandSource::default())
        .insert(Scanner {})
        .insert(EventSink::default())
        .insert(GravityScale(0.0))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(30.0, 50.0))
        .insert(Restitution::coefficient(0.1))
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 0.5,
        })
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            150.0 * (n as f32) + 10.0,
            300.0,
            0.0,
        )))
        .insert(ColliderMassProperties::Mass(1.0))
        .insert(ColliderMassProperties::Density(1.0))
}
