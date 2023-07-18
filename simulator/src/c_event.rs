use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use serde::Serialize;
use serde_json::{json, Value};

use crate::CollisionType;

#[derive(Component, Default)]
pub struct EventSink {
    pub queue: Vec<CTEvent>,
}

#[derive(Debug, Serialize)]
pub struct CTEvent {
    pub event_type: String,
    pub info: Value,
}

pub fn generate_event(
    event_type: String,
    event_sink: &mut EventSink,
    entity_of_interest: &Entity,
    transform_of_interest: &Transform,
    velocity_if_interest: Option<&Velocity>,
    collision_type: &CollisionType,
) {
    let zero = Velocity::zero();

    let vel = match velocity_if_interest {
        Some(x) => x,
        None => &zero,
    };
    let v = transform_of_interest.rotation * Vec3::Y;
    event_sink.queue.push(CTEvent {
        event_type,
        info: json!({
            "collision_type": format!("{:?}", collision_type),
            "entity": entity_of_interest,
            "transform": {
                "x": transform_of_interest.translation.x,
                "y": transform_of_interest.translation.y,
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
