use bevy_ecs::prelude::*;

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
    fn request_commands(&mut self) -> Vec<CCommand>;
    fn request_commands_by_event(&mut self, event: &Event) -> Vec<CCommand>;
}

pub struct DummyClient {}

impl ClientTrait for DummyClient {
    fn request_commands(&mut self) -> Vec<CCommand> {
        vec![CCommands::MOVE_FORWARD | CCommands::ROTATE_TANK_CLOCKWISE]
    }

    fn request_commands_by_event(&mut self, event: &Event) -> Vec<CCommand> {
        match event.event_type {
            EventType::Scan => {
                vec![CCommands::MOVE_FORWARD]
            }
            EventType::Hit => {
                vec![CCommands::MOVE_FORWARD]
            }
        }
    }
}

pub struct ReaderClient {
    pub lines: Vec<CCommand>,
}

impl ClientTrait for ReaderClient {
    fn request_commands(&mut self) -> Vec<CCommand> {
        if self.lines.is_empty() {
            vec![CCommands::NONE]
        } else {
            vec![self.lines.remove(0)]
        }
    }

    fn request_commands_by_event(&mut self, _event: &Event) -> Vec<CCommand> {
        if self.lines.is_empty() {
            vec![CCommands::NONE]
        } else {
            vec![self.lines.remove(0)]
        }
    }
}
