use bevy::prelude::*;
use serde::Serialize;
use serde_json::Value;

#[derive(Component)]
pub struct EventSink {
    pub queue: Vec<Event>,
}

impl EventSink {
    pub fn default() -> EventSink {
        EventSink { queue: Vec::new() }
    }
}

// pub const EVENT_TYPES_LENGTH: usize = 2;

#[derive(Debug, Serialize)]
pub struct Event {
    pub event_type: String,
    pub info: Value,
}
