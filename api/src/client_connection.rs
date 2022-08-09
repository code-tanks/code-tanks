use bevy_ecs::prelude::*;

use crate::{
    command_receiver::{CommandType, GroupedCommand, COMMAND_TYPES_LENGTH},
    event_sender::*,
};

#[derive(Component)]
pub struct ClientConnection {
    pub client: Box<dyn Client + Send + Sync>,
}

pub trait Defaults {
    fn dummy() -> ClientConnection;
}

impl Defaults for ClientConnection {
    fn dummy() -> ClientConnection {
        ClientConnection {
            client: Box::new(DummyClient {}),
        }
    }
}

pub trait Client {
    fn request_commands(&self) -> Vec<GroupedCommand>;
    fn request_commands_by_event(&self, event: &Event) -> Vec<GroupedCommand>;
}

pub struct DummyClient {}

impl Client for DummyClient {
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
