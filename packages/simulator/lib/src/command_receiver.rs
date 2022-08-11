use serde::{Deserialize, Serialize};
use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct CommandReceiver {
    pub queue: Vec<GroupedCommand>,
}

impl CommandReceiver {
    pub fn default() -> CommandReceiver {
        CommandReceiver { queue: Vec::new() }
    }
}


pub const COMMAND_TYPES_LENGTH: usize = 7;

#[derive(Debug, Serialize, Deserialize)] 
pub struct GroupedCommand {
    pub command_array: [u64; COMMAND_TYPES_LENGTH],
}
#[repr(u64)]
pub enum CommandType {
    None,
    MoveForward,
    MoveBackward,
    RotateTankClockwise,
    RotateTankCounterClockwise,
    RotateGunClockwise,
    RotateGunCounterClockwise,
    RotateRaderClockwise,
    RotateRaderCounterClockwise,
    FireWithPower,
}
