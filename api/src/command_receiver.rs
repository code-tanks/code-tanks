use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct CommandReceiver {
    pub queue: Vec<GroupedCommand>,
}

impl Default for CommandReceiver {
    fn default() -> CommandReceiver {
        CommandReceiver { queue: Vec::new() }
    }
}

// pub struct CommandQueue {
//     commands: Vec<Command>,
//     end_of_turn_commands: HashMap<String, Vec<Command>>,
//     num_commands: u32,
// }
pub const COMMAND_TYPES_LENGTH: usize = 7;

#[derive(Debug)]
pub struct GroupedCommand {
    // pub array = [0; 3];
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
