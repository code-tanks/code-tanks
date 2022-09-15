use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde_json::json;

use crate::{
    c_event::{Event, EventSink, EventTypes},
    c_health::Health,
    c_tank::{Bullet, Radar, Tank},
    CCollider, CollisionType,
};

pub fn bullet_physics(
    rapier_context: Res<RapierContext>,
    query_bullet: Query<Entity, With<Bullet>>,
    query_collidable: Query<Entity, (With<Collider>, Without<Bullet>, Without<Radar>)>,
    // mut query_health: Query<&mut Health>,
    mut commands: Commands,
    // state: Res<TickState>,
) {
    for a in query_collidable.iter() {
        for bullet in query_bullet.iter() {
            /* Find the intersection pair, if it exists, between two colliders. */
            if rapier_context.intersection_pair(a, bullet) == Some(true) {
                commands.entity(bullet).despawn();

                // if let Some(_) = tank {
                //     let mut health = query_health.get_mut(a).unwrap();
                //     health.val = health.val - 10;
                // }
            }
        }
    }
}

pub fn radar_physics(
    mut contact_events: EventReader<CollisionEvent>,
    // mut intersection_events: EventReader<IntersectionEvent>,
    mut query_tank: Query<(Entity, &Tank, &mut EventSink, &Transform)>,
    query_collider: Query<(&CCollider, &Transform)>,
    rapier_context: Res<RapierContext>,
    // state: Res<TickState>,
    query_bullet: Query<&Bullet>,
) {
    // for intersection_event in intersection_events.iter() {
    //     for (tank_entity, tank, mut event_sink, transform) in &mut query_tank {
    //         if intersection_event.collider1.entity() == tank.radar
    //             && intersection_event.collider1.entity() != tank_entity
    //         {
    //             // commands.entity(entity).despawn();

    //             let (collider, collider_transform) = query_collider
    //                 .get(intersection_event.collider2.entity())
    //                 .unwrap();
    //             // info!("{:?} {:?}", tank_entity, state.tick);
    //             info!(
    //                 "Tank Got Scan:{:?} Radar:{:?} Other:{:?}",
    //                 tank_entity,
    //                 tank.radar,
    //                 intersection_event.collider2.entity()
    //             );

    //             scan(
    //                 &tank_entity,
    //                 transform,
    //                 intersection_event.collider2.entity(),
    //                 &collider.collision_type,
    //                 &mut event_sink,
    //                 collider_transform,
    //                 &query_bullet,
    //             );
    //         } else if intersection_event.collider2.entity() == tank.radar
    //             && intersection_event.collider2.entity() != tank_entity
    //         {
    //             // commands.entity(entity).despawn();
    //             let (collider, collider_transform) = query_collider
    //                 .get(intersection_event.collider1.entity())
    //                 .unwrap();
    //             // info!("{:?} {:?}", tank_entity, state.tick);
    //             info!(
    //                 "Tank Got Scan:{:?} Radar:{:?} Other:{:?}",
    //                 tank_entity,
    //                 tank.radar,
    //                 intersection_event.collider1.entity()
    //             );

    //             scan(
    //                 &tank_entity,
    //                 transform,
    //                 intersection_event.collider1.entity(),
    //                 &collider.collision_type,
    //                 &mut event_sink,
    //                 collider_transform,
    //                 &query_bullet,
    //             );
    //         }
    //     }
    // }

    for contact_event in contact_events.iter() {
        for (tank_entity, tank, mut event_sink, transform) in &mut query_tank {
            // let radar = query_radar.get(tank.radar).unwrap();

            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                if h1 == &tank.radar && *h2 != tank_entity {
                    if rapier_context.intersection_pair(*h1, *h2) == Some(true) {
                        let (collider, collider_transform) = query_collider.get(*h2).unwrap();
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
                        );
                    }
                } else if h2 == &tank.radar && *h1 != tank_entity {
                    if rapier_context.intersection_pair(*h1, *h2) == Some(true) {
                        let (collider, collider_transform) = query_collider.get(*h1).unwrap();
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
    _t2: &Transform,
    query: &Query<&Bullet>,
) {
    if *collision_type == CollisionType::Bullet {
        let bullet = query.get(*b).unwrap();

        if bullet.tank == *a {
            return;
        }
    }
    info!("SCANNED {:?} of type {:?}", b, collision_type);

    event_sink.queue.push(Event {
        event_type: EventTypes::SCAN,
        info: json!({}), // TODO populate
    });
    // match *collision_type {
    //     CollisionType::Bullet => {
    //         let bullet = query.get(*b).unwrap();

    //         info!("Tank Got Scan:{:?}, Shot From:{:?}, eq:{:?}", a, bullet.tank, bullet.tank == *a);

    //         if bullet.tank == *a {
    //             return;
    //         }
    //     },
    //     _ => {

    //     }
    // }
}

pub fn tank_physics(
    mut contact_events: EventReader<CollisionEvent>,
    mut query_tank: Query<(Entity, &mut EventSink, &mut Health), With<Tank>>,
    query_collider: Query<(&CCollider, &Transform)>,
    // state: Res<TickState>,
    // mut commands: Commands,
) {
    for contact_event in contact_events.iter() {
        for (tank, mut event_sink, mut health) in &mut query_tank {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                if h1 == &tank {
                    let (collider, transform) = query_collider.get(*h2).unwrap();
                    hit(
                        &tank,
                        h2,
                        &collider.collision_type,
                        &mut event_sink,
                        &mut health,
                        transform,
                    );
                } else if h2 == &tank {
                    let (collider, transform) = query_collider.get(*h1).unwrap();
                    hit(
                        &tank,
                        h1,
                        &collider.collision_type,
                        &mut event_sink,
                        &mut health,
                        transform,
                    );
                }
            }
        }
    }
}

fn hit(
    tank: &Entity,
    b: &Entity,
    collision_type: &CollisionType,
    event_sink: &mut EventSink,
    health: &mut Health,
    _t: &Transform,
) {
    match collision_type {
        &CollisionType::Radar => {
            return;
        }
        _ => {}
    };

    info!("HIT {:?}, by {:?} of type {:?}", tank, b, collision_type);
    health.val = health.val - 10;

    if health.val < 0 {
        health.val = 0;
    }

    event_sink.queue.push(Event {
        event_type: EventTypes::HIT,
        info: json!({}), // TODO populate
    });
}
