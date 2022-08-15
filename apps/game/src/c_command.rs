use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// use serde::{Deserialize, Serialize};
use bevy::prelude::*;

#[derive(Component)]
pub struct CommandSource {
    pub queue: Vec<Command>,
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

pub type Command = u64;

pub enum Commands {}

impl Commands {
    pub const NONE: Command = 0b0;
    pub const MOVE_FORWARD: Command = 0b1;
    pub const MOVE_BACKWARD: Command = 0b10;
    pub const ROTATE_TANK_CLOCKWISE: Command = 0b100;
    pub const ROTATE_TANK_COUNTER_CLOCKWISE: Command = 0b1000;
    pub const ROTATE_GUN_CLOCKWISE: Command = 0b10000;
    pub const ROTATE_GUN_COUNTER_CLOCKWISE: Command = 0b100000;
    pub const FIRE_WITH_POWER: Command = 0b100000000;
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

impl Commands {
    pub fn all_commands_from_file(file: &str) -> Vec<Vec<Command>> {
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
                        .map(|f| f.parse::<Command>().unwrap())
                        .collect()
                } else {
                    Vec::new()
                }
            })
            .collect()
    }
}
