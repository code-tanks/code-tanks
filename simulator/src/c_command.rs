use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// use serde::{Deserialize, Serialize};
use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct CommandSource {
    pub queue: Vec<CCommand>,
}

impl CommandSource {
    pub fn default() -> CommandSource {
        CommandSource { queue: Vec::new() }
    }
}

// pub const COMMAND_TYPES_LENGTH: usize = 7;

// #[derive(Debug)]
// pub struct GroupedCommand {
//     pub command_array: [bool; COMMAND_TYPES_LENGTH],
// }

pub type CCommand = u64;

pub enum CCommands {}

impl CCommands {
    pub const NONE: CCommand = 0b0;
    pub const MOVE_FORWARD: CCommand = 0b1;
    pub const MOVE_BACKWARD: CCommand = 0b10;
    pub const ROTATE_TANK_CLOCKWISE: CCommand = 0b100;
    pub const ROTATE_TANK_COUNTER_CLOCKWISE: CCommand = 0b1000;
    pub const ROTATE_GUN_CLOCKWISE: CCommand = 0b10000;
    pub const ROTATE_GUN_COUNTER_CLOCKWISE: CCommand = 0b100000;
    pub const FIRE_WITH_POWER: CCommand = 0b100000000;
}

// #[repr(usize)]
// pub enum CommandType {
//     None = 00000000,
//     MoveForward,
//     MoveBackward,
//     RotateTankClockwise,
//     RotateTankCounterClockwise,
//     RotateGunClockwise,
//     RotateGunCounterClockwise,
//     RotateRaderClockwise,
//     RotateRaderCounterClockwise,
//     FireWithPower,
// }

impl CCommands {
    pub fn all_commands_from_file(file: &str) -> Vec<Vec<CCommand>> {
        let file = File::open(file).unwrap();
        let reader = BufReader::new(file);

        let lines: Vec<String> = reader
            .lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();

        let s: usize = lines[0].parse::<usize>().unwrap();

        (0..s)
            .map(|n| {
                if lines.len() > 1 + n {
                    lines[(1 + n)..]
                        .iter()
                        .step_by(s)
                        .map(|f| f.to_string())
                        .map(|f| f.parse::<CCommand>().unwrap())
                        .collect()
                } else {
                    Vec::new()
                }
            })
            .collect()
    }
}
