use bevy::prelude::Component;

use crate::{c_command::*, c_event::*};

#[derive(Component)]
pub struct Client {
    pub client: Box<dyn ClientTrait + Send + Sync>,
}

// impl Client {
//     // pub fn dummy() -> ClientConnection {
//     //     ClientConnection {
//     //         client: Box::new(DummyClient {}),
//     //     }
//     // }

//     // pub fn url(url: &str) -> Client {
//     //     Client {
//     //         client: Box::new(DummyClient {}),
//     //     }
//     // }

//     // pub fn reader(lines: Vec<String>) -> Client {
//     //     Client {
//     //         client: Box::new(ReaderClient { lines }),
//     //     }
//     // }
// }

pub trait ClientTrait {
    fn request_commands(&mut self) -> Vec<Command>;
    fn request_commands_by_event(&mut self, event: &Event) -> Vec<Command>;
}

pub struct DummyClient {}

impl ClientTrait for DummyClient {
    fn request_commands(&mut self) -> Vec<Command> {
        vec![Commands::MOVE_FORWARD | Commands::ROTATE_TANK_CLOCKWISE]
    }

    fn request_commands_by_event(&mut self, event: &Event) -> Vec<Command> {
        match event.event_type {
            EventType::Scan => {
                vec![Commands::MOVE_FORWARD]
            }
            EventType::Hit => {
                vec![Commands::MOVE_FORWARD]
            }
        }
    }
}

pub struct ReaderClient {
    pub lines: Vec<Command>,
}

impl ClientTrait for ReaderClient {
    fn request_commands(&mut self) -> Vec<Command> {
        vec![self.lines.remove(0)]
    }

    fn request_commands_by_event(&mut self, _event: &Event) -> Vec<Command> {
        vec![self.lines.remove(0)]
    }
}
