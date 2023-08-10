use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    c_event::{generate_event, EventSink},
    c_tank::{Bullet, Radar, Tank},
    CCollider, CollisionType, c_health::Health,
};

pub fn radar_physics(
    rapier_context: Res<RapierContext>,
    mut contact_events: EventReader<CollisionEvent>,
    mut query_tank: Query<(Entity, &Tank, &mut EventSink, &Transform)>,
    query_bullet: Query<&Bullet>,
    query_other_tank_health: Query<&Health, With<Tank>>,
    query_radar: Query<&mut Radar>,
    query_collider: Query<(&CCollider, &Transform, Option<&Velocity>)>,
) {
    for contact_event in contact_events.iter() {
        for (tank_entity, tank, mut event_sink, tank_transform) in &mut query_tank {
            let radar = query_radar.get(tank.radar).unwrap();

            if let CollisionEvent::Started(collision_entity_1, collision_entity_2, _event_flag) =
                contact_event
            {
                if !radar.disabled {
                    if collision_entity_1 == &tank.radar && *collision_entity_2 != tank_entity {
                        if rapier_context
                            .intersection_pair(*collision_entity_1, *collision_entity_2)
                            == Some(true)
                        {
                            let (collider, scanned_entity_transform, scanned_entity_velocity) =
                                query_collider.get(*collision_entity_2).unwrap();
                            // info!("{:?} {:?}", tank_entity, state.tick);
                            info!(
                                "Tank Got Scan:{:?} Radar:{:?} Other:{:?}",
                                tank_entity, tank.radar, collision_entity_2
                            );

                            on_radar_collision(
                                &mut event_sink,
                                &tank_entity,
                                tank_transform,
                                collision_entity_2,
                                scanned_entity_transform,
                                scanned_entity_velocity,
                                &collider.collision_type,
                                &query_bullet,
                                &query_other_tank_health,
                            );
                        }
                    } else if collision_entity_2 == &tank.radar
                        && *collision_entity_1 != tank_entity
                    {
                        if rapier_context
                            .intersection_pair(*collision_entity_1, *collision_entity_2)
                            == Some(true)
                        {
                            let (collider, collider_transform, velocity) =
                                query_collider.get(*collision_entity_1).unwrap();
                            // info!("{:?} {:?}", tank_entity, state.tick);
                            info!(
                                "Tank Got Scan:{:?} Radar:{:?} Other:{:?}",
                                tank_entity, tank.radar, collision_entity_1
                            );

                            on_radar_collision(
                                &mut event_sink,
                                &tank_entity,
                                tank_transform,
                                collision_entity_1,
                                collider_transform,
                                velocity,
                                &collider.collision_type,
                                &query_bullet,
                                &query_other_tank_health,
                            );
                        }
                    }
                }
            }
        }
    }
}

fn on_radar_collision(
    event_sink: &mut EventSink,
    tank_entity: &Entity,
    _tank_transform: &Transform,
    scanned_entity: &Entity,
    scanned_entity_transform: &Transform,
    scanned_entity_velocity: Option<&Velocity>,
    collision_type: &CollisionType,
    query_bullet: &Query<&Bullet>,
    query_other_tank_health: &Query<&Health, With<Tank>>,
) {
    if *collision_type == CollisionType::Bullet {
        let bullet = query_bullet.get(*scanned_entity).unwrap();

        if bullet.tank == *tank_entity {
            // SKIP SCAN IF BULLET WAS SHOT FROM SELF
            return;
        }
    } else if *collision_type == CollisionType::Tank {
        let other_tank_health = query_other_tank_health.get(*scanned_entity).unwrap();
        if other_tank_health.val <= 0 {
            // SKIP SCAN IF OTHER TANK IS DEAD
            return;
        }
    }

    info!("SCANNED {:?} of type {:?}", scanned_entity, collision_type);

    generate_event(
        "radar_scan".to_string(),
        event_sink,
        scanned_entity,
        scanned_entity_transform,
        scanned_entity_velocity,
        collision_type,
    );
}
