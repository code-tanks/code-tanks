use bevy_ecs::prelude::*;
// use serde::{Deserialize, Serialize};
// use serde_json::Value;

#[derive(Component)]
pub struct EventSink {
    pub queue: Vec<Event>,
}

impl EventSink {
    pub fn default() -> EventSink {
        EventSink { queue: Vec::new() }
    }
}

pub const EVENT_TYPES_LENGTH: usize = 2;

#[derive(Debug)]
pub struct Event {
    pub event_type: EventType,
    pub info: String,
}

#[repr(u64)]
#[derive(Debug)]
pub enum EventType {
    Scan,
    Hit,
}
