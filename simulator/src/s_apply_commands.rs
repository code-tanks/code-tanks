use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    c_command::{CCommands, CommandSource},
    collision_mask,
};

pub fn apply_commands(
    mut commands: Commands,
    mut query: Query<(
        &mut CommandSource,
        &Transform,
        &RigidBody,
        &mut Velocity, // &mut CVelocity,
                       // &mut TankVelocity,
                       // &Position,
    )>,
) {
    for (mut command_receiver, transform, _body, mut velocity) in &mut query {
        let grouped_commands = command_receiver.queue.remove(0);

        println!("apply_commands {:?}", grouped_commands);
        info!("apply_commands {:?}", grouped_commands);
        let mut vel = Vec2::ZERO;
        let mut ang = 0.0;
        if CCommands::MOVE_FORWARD & grouped_commands != 0 {
            let dir = transform.rotation * Vec3::Y;

            vel.x += 100.0 * dir.x;
            vel.y += 100.0 * dir.y;
        }
        if CCommands::MOVE_BACKWARD & grouped_commands != 0 {
            let dir = transform.rotation * Vec3::Y;
            vel.x -= 100.0 * dir.x;
            vel.y -= 100.0 * dir.y;
        }
        if CCommands::ROTATE_TANK_CLOCKWISE & grouped_commands != 0 {
            ang -= 0.25 * std::f32::consts::PI;
        }
        if CCommands::ROTATE_TANK_COUNTER_CLOCKWISE & grouped_commands != 0 {
            ang += 0.25 * std::f32::consts::PI;
        }

        if CCommands::FIRE & grouped_commands != 0 {
            let t = transform.rotation * Vec3::Y;
            commands
                .spawn()
                .insert(GravityScale(0.0))
                .insert(RigidBody::Dynamic)
                .insert(ColliderMassProperties::Mass(1.0))
                .insert(ColliderMassProperties::Density(1.0))
                .insert(Collider::cuboid(5.0, 5.0))
                .insert(Restitution::coefficient(0.1))
                .insert(CollisionGroups::new(
                    collision_mask::TANK,
                    collision_mask::ALL,
                ))
                .insert(Damping {
                    linear_damping: 0.0,
                    angular_damping: 0.0,
                })
                .insert(Velocity {
                    linvel: Vec2::new(t.x, t.y),
                    angvel: 0.0,
                })
                .insert_bundle(SpatialBundle {
                    transform: transform.with_translation(t),
                    // transform: Transform::from_xyz(10.0, 20.0, 30.0),
                    visibility: Visibility { is_visible: true },
                    ..default()
                });
        }

        velocity.linvel = vel;

        velocity.angvel = ang;

        println!("commands remaining {:?}", command_receiver.queue);
        info!("commands remaining {:?}", command_receiver.queue);
    }
}
