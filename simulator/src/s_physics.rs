use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    c_event::EventSink,
    c_health::Health,
    c_tank::{Bullet, Tank},
    CCollider, CollisionType,
};

pub fn physics(
    rapier_context: Res<RapierContext>,
    query_bullet: Query<Entity, With<Bullet>>,
    mut query_collidable: Query<(Entity, Option<&mut Health>), (With<Collider>, Without<Bullet>)>,
    mut commands: Commands,
) {
    for (a, mut health) in &mut query_collidable {
        for bullet in query_bullet.iter() {
            /* Find the intersection pair, if it exists, between two colliders. */
            if rapier_context.intersection_pair(a, bullet) == Some(true) {
                commands.entity(bullet).despawn();

                if health.is_some() {
                    health.val = health.val - 10;
                }
            }
        }
    }
}

pub fn physics2(
    mut contact_events: EventReader<CollisionEvent>,
    query_tank: Query<(Entity, &EventSink, &Health), With<Tank>>,
    query_collider: Query<&CCollider>,
    // mut commands: Commands,
) {
    for contact_event in contact_events.iter() {
        for (entity, _event_sink, _health) in query_tank.iter() {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                if h1 == &entity {
                    hit(
                        &entity,
                        h2,
                        &query_collider
                            .get_component::<CCollider>(*h2)
                            .unwrap()
                            .collision_type,
                    );
                } else if h2 == &entity {
                    hit(
                        &entity,
                        h1,
                        &query_collider
                            .get_component::<CCollider>(*h1)
                            .unwrap()
                            .collision_type,
                    );
                }
            }
        }
    }
}

fn hit(tank: &Entity, b: &Entity, collision_type: &CollisionType) {
    info!("HIT {:?}, by {:?} of type {:?}", tank, b, collision_type);
}
