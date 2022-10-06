use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    c_command::{CCommands, CommandSource},
    c_health::Health,
    c_tank::{Bullet, Gun, Radar, Tank},
    collision_mask, CCollider, CollisionType, TickState,
};

pub fn apply_commands(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut CommandSource,
            &Transform,
            &mut Velocity,
            &mut Tank,
            &Health,
        ),
        (Without<Radar>, Without<Gun>),
    >,
    mut query_radar: Query<
        (&mut Radar, &mut Transform, &mut Velocity),
        (Without<Gun>, Without<Tank>),
    >,
    mut query_gun: Query<
        (&mut Gun, &mut Transform, &mut Velocity),
        (Without<Radar>, Without<Tank>),
    >,
    mut state: ResMut<TickState>,
) {
    state.tick = state.tick + 1;

    for (entity, mut command_receiver, transform, mut velocity, mut tank, health) in &mut query {
        let mut vel = Vec2::ZERO;
        let mut ang = 0.0;
        velocity.linvel = vel;
        velocity.angvel = ang;

        let mut gun_ang = 0.0;
        let mut radar_ang = 0.0;
        let (mut radar, mut radar_transform, mut radar_velocity) =
            query_radar.get_mut(tank.radar).unwrap();
        let (mut gun, mut gun_transform, mut gun_velocity) = query_gun.get_mut(tank.gun).unwrap();
        radar_velocity.angvel = radar_ang;
        gun_velocity.angvel = gun_ang;

        gun_transform.translation.x = transform.translation.x;
        gun_transform.translation.y = transform.translation.y;

        radar_transform.translation.x = transform.translation.x;
        radar_transform.translation.y = transform.translation.y;

        if health.val == 0 {
            continue;
        }
        let grouped_commands = command_receiver.queue.remove(0);
        let rot = 3.14;

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
        if CCommands::LOCK_GUN & grouped_commands != 0 {
            gun.locked = true;
        }
        if CCommands::UNLOCK_GUN & grouped_commands != 0 {
            gun.locked = false;
        }
        if CCommands::LOCK_RADAR & grouped_commands != 0 {
            radar.locked = true;
        }
        if CCommands::UNLOCK_RADAR & grouped_commands != 0 {
            radar.locked = false;
        }
        if CCommands::ROTATE_TANK_CLOCKWISE & grouped_commands != 0 {
            ang -= 0.3 * rot;

            if gun.locked {
                gun_ang -= 0.3 * rot;

                if radar.locked {
                    radar_ang -= 0.3 * rot;
                }
            }
        }
        if CCommands::ROTATE_TANK_COUNTER_CLOCKWISE & grouped_commands != 0 {
            ang += 0.3 * rot;

            if gun.locked {
                gun_ang += 0.3 * rot;

                if radar.locked {
                    radar_ang += 0.3 * rot;
                }
            }
        }
        if CCommands::ROTATE_GUN_CLOCKWISE & grouped_commands != 0 {
            gun_ang -= 0.3 * rot;

            if radar.locked {
                radar_ang -= 0.3 * rot;
            }
        }
        if CCommands::ROTATE_GUN_COUNTER_CLOCKWISE & grouped_commands != 0 {
            gun_ang += 0.3 * rot;

            if radar.locked {
                radar_ang += 0.3 * rot;
            }
        }
        if CCommands::ROTATE_RADAR_CLOCKWISE & grouped_commands != 0 {
            radar_ang -= 0.3 * rot;
        }
        if CCommands::ROTATE_RADAR_COUNTER_CLOCKWISE & grouped_commands != 0 {
            radar_ang += 0.3 * rot;
        }
        if CCommands::FIRE & grouped_commands != 0 {
            if tank.cooldown <= 0 {
                let t = gun_transform.rotation * Vec3::Y;
                commands
                    .spawn()
                    .insert(CCollider {
                        collision_type: CollisionType::Bullet,
                    })
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(Sensor)
                    .insert(Bullet { tank: entity })
                    .insert(GravityScale(0.0))
                    .insert(RigidBody::Dynamic)
                    .insert(ColliderMassProperties::Mass(1.0))
                    .insert(ColliderMassProperties::Density(1.0))
                    .insert(Collider::ball(5.0))
                    .insert(Restitution::coefficient(0.1))
                    .insert(CollisionGroups::new(
                        Group::from_bits_truncate(collision_mask::BULLET),
                        Group::from_bits_truncate(
                            collision_mask::WALL | collision_mask::TANK | collision_mask::RADAR,
                        ),
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

        gun_velocity.angvel = gun_ang;
        radar_velocity.angvel = radar_ang;

        // let _v = transform.rotation * Vec3::Y;
        // info!("{:?}", v.y.atan2(v.x));
    }
}
