use bevy::prelude::{
    default, Commands as BevyCommands, Entity, Query, ResMut, SpatialBundle, Transform, Vec2, Vec3,
    Visibility, Without,
};
// use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use ct_api::Commands;
use serde_json::json;

use crate::{
    c_command_source::CommandSource,
    c_event::{CTEvent, EventSink},
    c_health::Health,
    c_tank::{Bullet, Gun, Radar, Tank},
    CCollider, CollisionMask, CollisionType, TickState,
};

pub fn apply_commands(
    mut commands: BevyCommands,
    mut query: Query<
        (
            Entity,
            &mut CommandSource,
            &Transform,
            &mut Velocity,
            &mut Tank,
            &mut Health,
            &mut EventSink,
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
    state.count += 1;

    for (
        entity,
        mut command_receiver,
        transform,
        mut velocity,
        mut tank,
        mut health,
        mut event_sink,
    ) in &mut query
    {
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
        if Commands::SELF_DESTRUCT & grouped_commands != 0 {
            health.val = 0;
            continue;
        }

        if Commands::MOVE_FORWARD & grouped_commands != 0 {
            let dir = transform.rotation * Vec3::Y;

            vel.x += Tank::MOVEMENT_SPEED * dir.x;
            vel.y += Tank::MOVEMENT_SPEED * dir.y;
        }
        if Commands::MOVE_BACKWARD & grouped_commands != 0 {
            let dir = transform.rotation * Vec3::Y;
            vel.x -= Tank::MOVEMENT_SPEED * dir.x;
            vel.y -= Tank::MOVEMENT_SPEED * dir.y;
        }
        if Commands::LOCK_GUN & grouped_commands != 0 {
            gun.locked = true;
        }
        if Commands::UNLOCK_GUN & grouped_commands != 0 {
            gun.locked = false;
        }
        if Commands::LOCK_RADAR & grouped_commands != 0 {
            radar.locked = true;
        }
        if Commands::UNLOCK_RADAR & grouped_commands != 0 {
            radar.locked = false;
        }
        if Commands::ROTATE_TANK_CLOCKWISE & grouped_commands != 0 {
            ang -= Tank::ROTATION_SPEED;

            if gun.locked {
                gun_ang -= Tank::ROTATION_SPEED;

                if radar.locked {
                    radar_ang -= Tank::ROTATION_SPEED;
                }
            }
        }
        if Commands::ROTATE_TANK_COUNTER_CLOCKWISE & grouped_commands != 0 {
            ang += Tank::ROTATION_SPEED;

            if gun.locked {
                gun_ang += Tank::ROTATION_SPEED;

                if radar.locked {
                    radar_ang += Tank::ROTATION_SPEED;
                }
            }
        }
        if Commands::ROTATE_GUN_CLOCKWISE & grouped_commands != 0 {
            gun_ang -= Tank::ROTATION_SPEED;

            if radar.locked {
                radar_ang -= Tank::ROTATION_SPEED;
            }
        }
        if Commands::ROTATE_GUN_COUNTER_CLOCKWISE & grouped_commands != 0 {
            gun_ang += Tank::ROTATION_SPEED;

            if radar.locked {
                radar_ang += Tank::ROTATION_SPEED;
            }
        }
        if Commands::ROTATE_RADAR_CLOCKWISE & grouped_commands != 0 {
            radar_ang -= Tank::ROTATION_SPEED;
        }
        if Commands::ROTATE_RADAR_COUNTER_CLOCKWISE & grouped_commands != 0 {
            radar_ang += Tank::ROTATION_SPEED;
        }
        if Commands::FIRE & grouped_commands != 0 && tank.cooldown == 0 {
            let t = gun_transform.rotation * Vec3::Y;
            commands.spawn((
                CCollider {
                    collision_type: CollisionType::Bullet,
                },
                ActiveEvents::COLLISION_EVENTS,
                Sensor,
                Bullet { tank: entity },
                GravityScale(0.0),
                RigidBody::Dynamic,
                // ColliderMassProperties::Mass(1.0),
                ColliderMassProperties::Density(1.0),
                Collider::ball(Bullet::RADIUS),
                Restitution::coefficient(0.1),
                CollisionGroups::new(
                    Group::from_bits_truncate(CollisionMask::BULLET),
                    Group::from_bits_truncate(
                        CollisionMask::WALL
                            | CollisionMask::TANK
                            | CollisionMask::RADAR
                            | CollisionMask::BULLET,
                    ),
                ),
                Damping {
                    linear_damping: 0.0,
                    angular_damping: 0.0,
                },
                Velocity {
                    linvel: Vec2::new(t.x * Bullet::SPEED, t.y * Bullet::SPEED),
                    angvel: 0.0,
                },
                SpatialBundle {
                    transform: {
                        let mut t = Transform::from_translation(
                            transform.translation
                                + t * Vec3::new(
                                    Tank::RADIUS + Bullet::RADIUS * 2.,
                                    Tank::RADIUS + Bullet::RADIUS * 2.,
                                    1.0,
                                ),
                        );
                        t.translation.z = 2.5;
                        t
                    },
                    visibility: Visibility::Visible,
                    ..default()
                },
            ));
            tank.cooldown = Tank::MAX_COOLDOWN;
        }

        if Commands::REQUEST_INFO & grouped_commands != 0 {
            let v = transform.rotation * Vec3::Y;
            let v2 = gun_transform.rotation * Vec3::Y;
            let v3 = radar_transform.rotation * Vec3::Y;

            event_sink.queue.push(CTEvent {
                event_type: "request_info".to_string(),
                info: json!({
                    "tank": {
                            "x": transform.translation.x,
                            "y": transform.translation.y,
                            "rotation": v.y.atan2(v.x),
                    },
                    "gun": {

                            "rotation": v2.y.atan2(v2.x),
                    },
                    "radar": {

                            "rotation": v3.y.atan2(v3.x),
                    }
                }),
            });
        }

        if Commands::DISABLE_RADAR & grouped_commands != 0 {
            radar.disabled = true;
        }

        if Commands::ENABLE_RADAR & grouped_commands != 0 {
            radar.disabled = false;
        }

        if Commands::CLEAR_COMMANDS & grouped_commands != 0 {
            command_receiver.queue.clear();
        }

        if tank.cooldown > 0 {
            tank.cooldown -= 1;
        }

        velocity.linvel = vel;
        velocity.angvel = ang;
        gun_velocity.linvel = vel;
        radar_velocity.linvel = vel;

        gun_velocity.angvel = gun_ang;
        radar_velocity.angvel = radar_ang;

        // let _v = transform.rotation * Vec3::Y;
        // let _v1 = gun_transform.rotation * Vec3::Y;
        // let _v2 = radar_transform.rotation * Vec3::Y;
        // println!("angle: {} {} {}", _v.y.atan2(_v.x), _v2.y.atan2(_v.x), _v2.y.atan2(_v.x));
    }
}
