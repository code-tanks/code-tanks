use std::f64::consts::TAU;

use bevy_ecs::system::Query;

use crate::{
    c_collider::Collider,
    c_position::Position,
    c_velocity::{TankVelocity, Velocity},
};

pub fn physics(mut query: Query<(&mut Velocity, &mut Position, &Collider, &mut TankVelocity)>) {
    for (mut velocity, mut position, collider, mut tank_velocity) in &mut query {
        // physComp
        // ..position.features[0] += physComp.velocity * -sin(physComp.rotation)
        // ..position.features[1] += physComp.velocity * cos(physComp.rotation)
        // ..rotation = (physComp.rotation + rotationDelta) % tau
        // ..velocity *= 0
        // ..angularVelocity *= 0;
        position.x += velocity.velocity * -tank_velocity.angular_velocity.sin();
        position.y += velocity.velocity * tank_velocity.angular_velocity.cos();
        position.rotation = (position.rotation + tank_velocity.angular_velocity) % TAU;
        velocity.velocity = 0.0;
        tank_velocity.angular_velocity = 0.0;
    }
}
