use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde_json::json;

use crate::{
    c_event::{Event, EventSink},
    c_health::Health,
    c_tank::{Bullet, DamageDealer, Radar, Tank},
    CCollider, CollisionType,
};

pub fn bullet_physics(
    rapier_context: Res<RapierContext>,
    query_bullet: Query<Entity, With<Bullet>>,
    query_collidable: Query<Entity, (With<Collider>, Without<Bullet>, Without<Radar>)>,
    mut commands: Commands,
) {
    for a in query_collidable.iter() {
        for bullet in query_bullet.iter() {
            /* Find the intersection pair, if it exists, between two colliders. */
            if rapier_context.intersection_pair(a, bullet) == Some(true) {
                commands.entity(bullet).despawn();
            }
        }
    }
}

pub fn radar_physics(
    mut contact_events: EventReader<CollisionEvent>,
    mut query_tank: Query<(Entity, &Tank, &mut EventSink, &Transform)>,
    query_collider: Query<(&CCollider, &Transform, Option<&Velocity>)>,
    rapier_context: Res<RapierContext>,
    query_bullet: Query<&Bullet>,
) {
    for contact_event in contact_events.iter() {
        for (tank_entity, tank, mut event_sink, transform) in &mut query_tank {
            // let radar = query_radar.get(tank.radar).unwrap();

            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                if h1 == &tank.radar && *h2 != tank_entity {
                    if rapier_context.intersection_pair(*h1, *h2) == Some(true) {
                        let (collider, collider_transform, velocity) =
                            query_collider.get(*h2).unwrap();
                        // info!("{:?} {:?}", tank_entity, state.tick);
                        info!(
                            "Tank Got Scan:{:?} Radar:{:?} Other:{:?}",
                            tank_entity, tank.radar, h2
                        );

                        scan(
                            &tank_entity,
                            transform,
                            h2,
                            &collider.collision_type,
                            &mut event_sink,
                            collider_transform,
                            &query_bullet,
                            velocity,
                        );
                    }
                } else if h2 == &tank.radar && *h1 != tank_entity {
                    if rapier_context.intersection_pair(*h1, *h2) == Some(true) {
                        let (collider, collider_transform, velocity) =
                            query_collider.get(*h1).unwrap();
                        // info!("{:?} {:?}", tank_entity, state.tick);
                        info!(
                            "Tank Got Scan:{:?} Radar:{:?} Other:{:?}",
                            tank_entity, tank.radar, h1
                        );

                        scan(
                            &tank_entity,
                            transform,
                            h1,
                            &collider.collision_type,
                            &mut event_sink,
                            collider_transform,
                            &query_bullet,
                            velocity,
                        );
                    }
                }
            }
        }
    }
}

fn scan(
    a: &Entity,
    _t1: &Transform,
    b: &Entity,
    collision_type: &CollisionType,
    event_sink: &mut EventSink,
    t2: &Transform,
    query: &Query<&Bullet>,
    t2_velocity: Option<&Velocity>,
) {
    if *collision_type == CollisionType::Bullet {
        let bullet = query.get(*b).unwrap();

        if bullet.tank == *a {
            return;
        }
    }
    info!("SCANNED {:?} of type {:?}", b, collision_type);

    let v = t2.rotation * Vec3::Y;

    let zero = Velocity::zero();

    let vel = match t2_velocity {
        Some(x) => x,
        None => &zero,
    };

    event_sink.queue.push(Event {
        event_type: "scan".to_string(),
        info: json!({
            "collision_type": format!("{:?}", collision_type),
            "entity": b,
            "transform": {
                "x": t2.translation.x,
                "y": t2.translation.y,
                "rotation": v.y.atan2(v.x),
            },
            "velocity": {
                "linvel": {
                    "x": vel.linvel.x,
                    "y": vel.linvel.y
                },
                "angvel": vel.angvel
            }
        }),
    });
}

pub fn tank_physics(
    mut contact_events: EventReader<CollisionEvent>,
    mut query_tank_many: Query<(Entity, &Tank, &mut Health, &Transform, &Velocity)>,
    mut query_event_sink: Query<&mut EventSink>,
    mut query_damage_dealer: Query<&mut DamageDealer>,
    query_bullet: Query<&Bullet>,
    query_collider: Query<(&CCollider, &Transform, Option<&Velocity>)>,
) {
    for contact_event in contact_events.iter() {
        for (tank_entity, tank, mut health, transform, velocity) in &mut query_tank_many {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                if h1 == &tank_entity {
                    let (collider, transform2, velocity2) = query_collider.get(*h2).unwrap();
                    hit(
                        &tank_entity,
                        &tank,
                        &transform,
                        &velocity,
                        h2,
                        &collider.collision_type,
                        // &mut query_event_sink.get_mut(*h1).unwrap(),
                        &mut health,
                        transform2,
                        velocity2,
                        &mut query_damage_dealer,
                        &query_bullet,
                        &mut query_event_sink,
                    );
                } else if h2 == &tank_entity {
                    let (collider, transform2, velocity2) = query_collider.get(*h1).unwrap();
                    hit(
                        &tank_entity,
                        &tank,
                        &transform,
                        &velocity,
                        h1,
                        &collider.collision_type,
                        // &mut query_event_sink.get_mut(*h2).unwrap(),
                        &mut health,
                        transform2,
                        velocity2,
                        &mut query_damage_dealer,
                        &query_bullet,
                        &mut query_event_sink,
                    );
                }
            }
        }
    }
}

fn hit(
    tank_entity: &Entity,
    _tank: &Tank,
    transform: &Transform,
    velocity: &Velocity,
    collided_entity: &Entity,
    collision_type: &CollisionType,
    // event_sink: &mut EventSink,
    health: &mut Health,
    collided_entity_transform: &Transform,
    collided_entity_velocity: Option<&Velocity>,
    query_damage_dealer: &mut Query<&mut DamageDealer>,
    query_bullet: &Query<&Bullet>,
    query_event_sink: &mut Query<&mut EventSink>,
) {
    match collision_type {
        &CollisionType::Radar => {
            return;
        }
        &CollisionType::Bullet => {
            let damage_dealer = &mut query_damage_dealer
                .get_mut(query_bullet.get(*collided_entity).unwrap().tank)
                .unwrap();
            damage_dealer.damage_dealt += 10;
            let v = transform.rotation * Vec3::Y;
            // let zero = Velocity::zero();
        
            let mut event_sink = query_event_sink.get_mut(*tank_entity).unwrap();
            event_sink.queue.push(Event {
                event_type: "bullet hit".to_string(),
                info: json!({
                    "collision_type": format!("{:?}", CollisionType::Tank),
                    "entity": tank_entity,
                    "transform": {
                        "x": transform.translation.x,
                        "y": transform.translation.y,
                        "rotation": v.y.atan2(v.x),
                    },
                    "velocity": {
                        "linvel": {
                            "x": velocity.linvel.x,
                            "y": velocity.linvel.y
                        },
                        "angvel": velocity.angvel
                    }
                }), // TODO populate
            });
        }
        _ => {}
    };

    println!(
        "HIT {:?}, by {:?} of type {:?}",
        tank_entity, collided_entity, collision_type
    );
    health.val = health.val - 10;

    if health.val < 0 {
        health.val = 0;
    }
    println!("{:?}", health.val);

    let v = collided_entity_transform.rotation * Vec3::Y;
    let zero = Velocity::zero();

    let vel = match collided_entity_velocity {
        Some(x) => x,
        None => &zero,
    };
    let mut event_sink = query_event_sink.get_mut(*tank_entity).unwrap();
    event_sink.queue.push(Event {
        event_type: "hit".to_string(),
        info: json!({
            "collision_type": format!("{:?}", collision_type),
            "entity": collided_entity,
            "transform": {
                "x": collided_entity_transform.translation.x,
                "y": collided_entity_transform.translation.y,
                "rotation": v.y.atan2(v.x),
            },
            "velocity": {
                "linvel": {
                    "x": vel.linvel.x,
                    "y": vel.linvel.y
                },
                "angvel": vel.angvel
            }
        }), // TODO populate
    });
}
