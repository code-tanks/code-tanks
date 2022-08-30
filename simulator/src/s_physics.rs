use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    c_event::{Event, EventSink, EventTypes},
    c_health::Health,
    c_tank::{Bullet, Radar, Tank},
    CCollider, CollisionType, TickState,
};

pub fn bullet_physics(
    rapier_context: Res<RapierContext>,
    query_bullet: Query<Entity, With<Bullet>>,
    query_collidable: Query<Entity, (With<Collider>, Without<Bullet>, Without<Radar>)>,
    // mut query_health: Query<&mut Health>,
    mut commands: Commands,
    state: Res<TickState>,
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
    mut query_tank: Query<(Entity, &Tank, &mut EventSink, &Transform)>,
    query_collider: Query<(&CCollider, &Transform)>,
    state: Res<TickState>,
) {
    for contact_event in contact_events.iter() {
        for (tank_entity, tank, mut event_sink, transform) in &mut query_tank {
            // let radar = query_radar.get(tank.radar).unwrap();

            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                if h1 == &tank.radar && *h2 != tank_entity {
                    let (collider, collider_transform) = query_collider.get(*h2).unwrap();
                    info!("{:?} {:?}", tank_entity, state.tick);

                    scan(
                        transform,
                        h2,
                        &collider.collision_type,
                        &mut event_sink,
                        collider_transform,
                    );
                } else if h2 == &tank.radar && *h1 != tank_entity {
                    let (collider, collider_transform) = query_collider.get(*h1).unwrap();
                    info!("{:?} {:?}", tank_entity, state.tick);

                    scan(
                        transform,
                        h1,
                        &collider.collision_type,
                        &mut event_sink,
                        collider_transform,
                    );
                }
            }
        }
    }
}

fn scan(
    _t1: &Transform,
    b: &Entity,
    collision_type: &CollisionType,
    event_sink: &mut EventSink,
    _t2: &Transform,
) {
    info!("SCANNED {:?} of type {:?}", b, collision_type);

    event_sink.queue.push(Event {
        event_type: EventTypes::SCAN,
        info: "".to_string(),
    });
}

pub fn tank_physics(
    mut contact_events: EventReader<CollisionEvent>,
    mut query_tank: Query<(Entity, &mut EventSink, &mut Health), With<Tank>>,
    query_collider: Query<(&CCollider, &Transform)>,
    state: Res<TickState>,
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
        info: "".to_string(),
    });
}
