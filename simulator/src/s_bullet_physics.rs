use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    c_event::{generate_event, EventSink},
    c_tank::{Bullet, Radar},
    CCollider,
};

pub fn bullet_physics(
    rapier_context: Res<RapierContext>,
    mut commands: Commands,
    mut query_event_sink: Query<&mut EventSink>,
    query_bullet: Query<(Entity, &Bullet)>,
    query_collidable: Query<(Entity, &CCollider, &Transform, Option<&Velocity>), Without<Radar>>,
) {
    for (collision_entity, ccollider, collision_entity_transform, collision_entity_velocity) in
        query_collidable.iter()
    {
        for (bullet_entity, bullet) in query_bullet.iter() {
            /* Find the intersection pair, if it exists, between two colliders. */
            if rapier_context.intersection_pair(collision_entity, bullet_entity) == Some(true) {
                generate_event(
                    "bullet_hit".to_string(),
                    &mut query_event_sink.get_mut(bullet.tank).unwrap(),
                    &collision_entity,
                    collision_entity_transform,
                    collision_entity_velocity,
                    &ccollider.collision_type,
                );
                commands.entity(bullet_entity).despawn_recursive();
            }
        }
    }
}
