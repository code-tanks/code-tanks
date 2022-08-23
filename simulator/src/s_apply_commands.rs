use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    c_command::{CCommands, CommandSource},
    c_tank::{Bullet, Tank},
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
        &mut Tank,
    )>,
) {
    for (mut command_receiver, transform, _body, mut velocity, mut tank) in &mut query {
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
            ang -= 0.3 * std::f32::consts::PI;
        }
        if CCommands::ROTATE_TANK_COUNTER_CLOCKWISE & grouped_commands != 0 {
            ang += 0.3 * std::f32::consts::PI;
        }

        if CCommands::FIRE & grouped_commands != 0 {
            if tank.cooldown == 0 {
                let t = transform.rotation * Vec3::Y;
                commands
                    .spawn()
                    .insert(Bullet {})
                    .insert(Sensor(true))
                    .insert(GravityScale(0.0))
                    .insert(RigidBody::Dynamic)
                    .insert(ColliderMassProperties::Mass(1.0))
                    .insert(ColliderMassProperties::Density(1.0))
                    .insert(Collider::ball(5.0))
                    .insert(Restitution::coefficient(0.1))
                    .insert(CollisionGroups::new(
                        collision_mask::BULLET,
                        collision_mask::ALL,
                    ))
                    .insert(Damping {
                        linear_damping: 0.0,
                        angular_damping: 0.0,
                    })
                    .insert(Velocity {
                        linvel: Vec2::new(t.x * 200.0, t.y * 200.0),
                        angvel: 0.0,
                    })
                    .insert_bundle(SpatialBundle {
                        // transform: transform + t,
                        transform: Transform::from_translation(
                            transform.translation + t * Vec3::new(40.0, 40.0, 40.0),
                        ),
                        // transform: Transform::from_xyz(10.0, 20.0, 30.0),
                        visibility: Visibility { is_visible: true },
                        ..default()
                    });
                tank.cooldown = Tank::MAX_COOLDOWN;
            } else {
                tank.cooldown = tank.cooldown - 1;
            }
        }

        velocity.linvel = vel;

        velocity.angvel = ang;

        println!("commands remaining {:?}", command_receiver.queue);
        info!("commands remaining {:?}", command_receiver.queue);
    }
}
