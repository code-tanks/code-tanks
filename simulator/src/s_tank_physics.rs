use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    c_event::{generate_event, EventSink},
    c_health::Health,
    c_tank::{Bullet, DamageDealer, Tank},
    CCollider, CollisionType,
};

pub fn tank_physics(
    mut contact_events: EventReader<CollisionEvent>,
    mut query_tank_many: Query<(Entity, &Tank, &mut Health, &Transform, &Velocity)>,
    mut query_event_sink: Query<&mut EventSink>,
    mut query_damage_dealer: Query<&mut DamageDealer>,
    query_bullet: Query<&Bullet>,
    query_collidable: Query<(&CCollider, &Transform, Option<&Velocity>)>,
) {
    for contact_event in contact_events.iter() {
        for (tank_entity, tank, mut tank_health, tank_transform, tank_velocity) in
            &mut query_tank_many
        {
            if let CollisionEvent::Started(collision_entity_1, collision_entity_2, _event_flag) =
                contact_event
            {
                if collision_entity_1 == &tank_entity {
                    let (collider, collided_entity_transform, collided_entity_velocity) =
                        query_collidable.get(*collision_entity_2).unwrap();
                    on_tank_collision(
                        &tank_entity,
                        &tank,
                        &tank_transform,
                        &tank_velocity,
                        &mut tank_health,
                        collision_entity_2,
                        collided_entity_transform,
                        collided_entity_velocity,
                        &collider.collision_type,
                        &mut query_damage_dealer,
                        &query_bullet,
                        &mut query_event_sink,
                    );
                } else if collision_entity_2 == &tank_entity {
                    let (collider, collided_entity_transform, collided_entity_velocity) =
                        query_collidable.get(*collision_entity_1).unwrap();
                    on_tank_collision(
                        &tank_entity,
                        &tank,
                        &tank_transform,
                        &tank_velocity,
                        &mut tank_health,
                        collision_entity_1,
                        collided_entity_transform,
                        collided_entity_velocity,
                        &collider.collision_type,
                        &mut query_damage_dealer,
                        &query_bullet,
                        &mut query_event_sink,
                    );
                }
            }
        }
    }
}

fn on_tank_collision(
    tank_entity: &Entity,
    _tank: &Tank,
    tank_transform: &Transform,
    tank_velocity: &Velocity,
    tank_health: &mut Health,
    collided_entity: &Entity,
    collided_entity_transform: &Transform,
    collided_entity_velocity: Option<&Velocity>,
    collision_type: &CollisionType,
    query_damage_dealer: &mut Query<&mut DamageDealer>,
    query_bullet: &Query<&Bullet>,
    query_event_sink: &mut Query<&mut EventSink>,
) {
    match collision_type {
        &CollisionType::Radar => {
            return;
        }
        &CollisionType::Bullet => {
            let tank_entity_that_shot_this_bullet =
                query_bullet.get(*collided_entity).unwrap().tank;
            let damage_dealer = &mut query_damage_dealer
                .get_mut(tank_entity_that_shot_this_bullet)
                .unwrap();
            damage_dealer.damage_dealt += 10;

            generate_event(
                "bullet_hit".to_string(),
                &mut query_event_sink
                    .get_mut(tank_entity_that_shot_this_bullet)
                    .unwrap(),
                tank_entity,
                tank_transform,
                Some(tank_velocity),
                &CollisionType::Tank,
            );
        }
        _ => {}
    };

    println!(
        "HIT {:?}, by {:?} of type {:?}",
        tank_entity, collided_entity, collision_type
    );
    tank_health.val = tank_health.val - 10;

    if tank_health.val < 0 {
        tank_health.val = 0;
    }

    generate_event(
        "tank_hit".to_string(),
        &mut query_event_sink.get_mut(*tank_entity).unwrap(),
        collided_entity,
        collided_entity_transform,
        collided_entity_velocity,
        collision_type,
    );
}
