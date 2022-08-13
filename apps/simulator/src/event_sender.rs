use bevy_ecs::prelude::Component;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Component)]
pub struct EventSender {
    pub queue: Vec<Event>,
}

impl EventSender {
    pub fn default() -> EventSender {
        EventSender { queue: Vec::new() }
    }
}

pub const EVENT_TYPES_LENGTH: usize = 2;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub event_type: EventType,
    pub info: Value,
}

#[repr(u64)]
#[derive(Debug, Serialize, Deserialize)]
pub enum EventType {
    Scan,
    Hit,
}
