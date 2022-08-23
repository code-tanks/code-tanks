use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    c_event::EventSink,
    c_health::Health,
    c_tank::{Bullet, Tank},
};

pub fn physics(
    rapier_context: Res<RapierContext>,
    query_bullet: Query<Entity, With<Bullet>>,
    query_collidable: Query<Entity, (With<Collider>, Without<Bullet>)>,
    // query: Query<(Entity, &Health, Option<&EventSink>, Option<&Bullet>)>,
    mut commands: Commands,
) {
    // for collision_event in collision_events.iter() {
    //     info!("{:?}", collision_event);
    //     for (entity, health, event_sink, bullet) in query.iter() {
    //         if let CollisionEvent::Started(a, b, _event_flag) = collision_event {
    //             if a == &entity {
    //                 check(a, health, event_sink, bullet, &mut commands);
    //             } else if b == &entity {
    //                 check(b, health, event_sink, bullet, &mut commands);
    //             }
    //         }
    //     }
    // }
    for a in query_collidable.iter() {
        for bullet in query_bullet.iter() {
            /* Find the intersection pair, if it exists, between two colliders. */
            if rapier_context.intersection_pair(a, bullet) == Some(true) {
                commands.entity(bullet).despawn();
            }
        }
        // for (b, b_transform, b_health, b_event_sink) in query_collidable.iter() {
        //     if rapier_context.intersection_pair(a, b) == Some(true) {
        //         hit(a, b, a_transform, b_transform, a_health, a_event_sink);
        //         hit(b, a, b_transform, a_transform, b_health, b_event_sink);
        //     }
        // }
    }
}

pub fn physics2(
    mut contact_events: EventReader<CollisionEvent>,
    query_tank: Query<(Entity, &EventSink, &Health), With<Tank>>,
    query_bullet: Query<Entity, With<Bullet>>,
    // mut commands: Commands,
) {
    for contact_event in contact_events.iter() {
        for (entity, _event_sink, _health) in query_tank.iter() {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                if h1 == &entity || h2 == &entity {
                    if let Ok(_) = query_bullet.get(*h1) {
                        continue;
                    }
                    if let Ok(_) = query_bullet.get(*h2) {
                        continue;
                    }
                    info!("HIT {:?}", entity);
                }
            }
        }
    }
}

// fn hit(
//     a: Entity,
//     _b: Entity,
//     a_transform: &Transform,
//     _b_transform: &Transform,
//     _health: Option<&Health>,
//     _event_sink: Option<&EventSink>,
// ) {
//     info!("HIT {:?} {:?}", a, a_transform);
// }

// fn check(
//     entity: &Entity,
//     _health: &Health,
//     event_sink: Option<&EventSink>,
//     bullet: Option<&Bullet>,
//     commands: &mut Commands,
// ) {
//     if let Some(_bullet) = bullet {
//         commands.entity(*entity).despawn();
//     } else if let Some(_event_sink) = event_sink {
//     }
// }
