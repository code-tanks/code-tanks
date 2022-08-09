use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct EventSender {
    pub queue: Vec<Event>,
}

impl Default for EventSender {
    fn default() -> EventSender {
        EventSender { queue: Vec::new() }
    }
}

pub const EVENT_TYPES_LENGTH: usize = 2;

#[derive(Debug)]
pub struct Event {
    pub event_type: EventType,
}

#[repr(u64)]
#[derive(Debug)]
pub enum EventType {
    Scan,
    Hit,
}
