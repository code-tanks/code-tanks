use bevy_ecs::prelude::*;

use crate::command_receiver::{CommandType, GroupedCommand, COMMAND_TYPES_LENGTH};

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
    fn push(&self);

    fn request_commands(&self) -> Vec<GroupedCommand>;
}

pub struct DummyClient {}

impl Client for DummyClient {
    fn push(&self) {}
    fn request_commands(&self) -> Vec<GroupedCommand> {
        let mut command_array: [u64; COMMAND_TYPES_LENGTH] =
            [CommandType::None as u64; COMMAND_TYPES_LENGTH];

        command_array[CommandType::MoveForward as usize] = 1;

        vec![GroupedCommand {
            command_array: command_array,
        }]
    }
}
