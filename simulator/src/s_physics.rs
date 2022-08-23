use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{c_event::EventSink, c_health::Health, c_tank::Bullet};

pub fn physics(
    query: Query<(Entity, &Health, Option<&EventSink>, Option<&Bullet>)>,
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.iter() {
        info!("{:?}", collision_event);
        for (entity, health, event_sink, bullet) in query.iter() {
            if let CollisionEvent::Started(a, b, _event_flag) = collision_event {
                if a == &entity {
                    check(a, health, event_sink, bullet, &mut commands);
                } else if b == &entity {
                    check(b, health, event_sink, bullet, &mut commands);
                }
            }
        }
    }
}

fn check(
    entity: &Entity,
    _health: &Health,
    event_sink: Option<&EventSink>,
    bullet: Option<&Bullet>,
    commands: &mut Commands,
) {
    if let Some(_bullet) = bullet {
        commands.entity(*entity).despawn();
    } else if let Some(_event_sink) = event_sink {
    }
}
