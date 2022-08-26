use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    c_command::{CCommands, CommandSource},
    c_health::Health,
    c_tank::{Bullet, Tank},
    collision_mask, CCollider, CollisionType,
};

pub fn apply_commands(
    mut commands: Commands,
    mut query: Query<(
        &mut CommandSource,
        &Transform,
        &mut Velocity,
        &mut Tank,
        &Health,
    )>,
) {
    for (mut command_receiver, transform, mut velocity, mut tank, health) in &mut query {
        let mut vel = Vec2::ZERO;
        let mut ang = 0.0;
        velocity.linvel = vel;
        velocity.angvel = ang;

        if health.val == 0 {
            continue;
        }
        let grouped_commands = command_receiver.queue.remove(0);

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
            if tank.cooldown <= 0 {
                let t = transform.rotation * Vec3::Y;
                commands
                    .spawn()
                    .insert(CCollider {
                        collision_type: CollisionType::Bullet,
                    })
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(Sensor)
                    .insert(Bullet {})
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
                        transform: Transform::from_translation(
                            transform.translation + t * Vec3::new(35.0, 35.0, 35.0),
                        ),
                        visibility: Visibility { is_visible: true },
                        ..default()
                    });
                tank.cooldown = Tank::MAX_COOLDOWN;
            }
        }
        if tank.cooldown > 0 {
            tank.cooldown = tank.cooldown - 1;
        }

        velocity.linvel = vel;

        velocity.angvel = ang;
    }
}
