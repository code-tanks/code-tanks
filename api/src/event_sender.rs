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

pub struct Event {}
