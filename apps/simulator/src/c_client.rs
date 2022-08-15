use bevy_ecs::prelude::Component;

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
    fn request_commands(&self) -> Vec<Command>;
    fn request_commands_by_event(&self, event: &Event) -> Vec<Command>;
}

pub struct DummyClient {}

impl ClientTrait for DummyClient {
    fn request_commands(&self) -> Vec<Command> {
        let cmd: Command = Commands::MOVE_FORWARD;

        vec![cmd]
    }

    fn request_commands_by_event(&self, event: &Event) -> Vec<Command> {
        let cmd: Command = Commands::MOVE_FORWARD;

        match event.event_type {
            EventType::Scan => {
                vec![cmd]
            }
            EventType::Hit => {
                vec![cmd]
            }
        }
    }
}

pub struct ReaderClient {
    pub lines: Vec<String>,
}

impl ClientTrait for ReaderClient {
    fn request_commands(&self) -> Vec<Command> {
        todo!()
    }

    fn request_commands_by_event(&self, event: &Event) -> Vec<Command> {
        todo!()
    }
}
