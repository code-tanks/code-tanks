use bevy_ecs::prelude::*;

use crate::{
    c_command::{CommandType, GroupedCommand, COMMAND_TYPES_LENGTH},
    c_event::*,
    my_reader::BufReader,
};

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
    fn request_commands(&self) -> Vec<GroupedCommand>;
    fn request_commands_by_event(&self, event: &Event) -> Vec<GroupedCommand>;
}

pub struct DummyClient {}

impl ClientTrait for DummyClient {
    fn request_commands(&self) -> Vec<GroupedCommand> {
        let mut command_array: [u64; COMMAND_TYPES_LENGTH] =
            [CommandType::None as u64; COMMAND_TYPES_LENGTH];

        command_array[CommandType::MoveForward as usize] = 1;
        command_array[CommandType::RotateTankClockwise as usize] = 1;

        vec![GroupedCommand {
            command_array: command_array,
        }]
    }

    fn request_commands_by_event(&self, event: &Event) -> Vec<GroupedCommand> {
        let mut command_array: [u64; COMMAND_TYPES_LENGTH] =
            [CommandType::None as u64; COMMAND_TYPES_LENGTH];

        match event.event_type {
            EventType::Scan => {
                command_array[CommandType::MoveBackward as usize] = 1;

                vec![GroupedCommand {
                    command_array: command_array,
                }]
            }
            EventType::Hit => {
                command_array[CommandType::MoveBackward as usize] = 1;

                vec![GroupedCommand {
                    command_array: command_array,
                }]
            }
        }
    }
}

pub struct ReaderClient {
    pub lines: Vec<String>,
}

impl ClientTrait for ReaderClient {
    fn request_commands(&self) -> Vec<GroupedCommand> {
        todo!()
    }

    fn request_commands_by_event(&self, event: &Event) -> Vec<GroupedCommand> {
        todo!()
    }
}
