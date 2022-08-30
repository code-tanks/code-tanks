use bevy::prelude::*;

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

#[derive(Debug)]
pub struct Event {
    pub event_type: EventType,
    pub info: String,
}

pub type EventType = u64;

pub enum EventTypes {}

impl EventTypes {
    pub const HIT: EventType = 0b1;
    pub const SCAN: EventType = 0b1 << 1;
}