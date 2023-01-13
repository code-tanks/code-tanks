use bevy::prelude::*;

#[derive(Component)]
pub struct CommandSource {
    pub queue: Vec<CCommand>,
}

impl CommandSource {
    pub fn default() -> CommandSource {
        CommandSource { queue: Vec::new() }
    }
}

pub type CCommand = u64;

pub enum CCommands {}

impl CCommands {
    pub const NONE: CCommand = 0b0;
    pub const MOVE_FORWARD: CCommand = 0b1;
    pub const MOVE_BACKWARD: CCommand = 0b1 << 1;
    pub const ROTATE_TANK_CLOCKWISE: CCommand = 0b1 << 2;
    pub const ROTATE_TANK_COUNTER_CLOCKWISE: CCommand = 0b1 << 3;
    pub const FIRE: CCommand = 0b1 << 4;
    pub const ROTATE_GUN_CLOCKWISE: CCommand = 0b1 << 5;
    pub const ROTATE_GUN_COUNTER_CLOCKWISE: CCommand = 0b1 << 6;
    pub const ROTATE_RADAR_CLOCKWISE: CCommand = 0b1 << 7;
    pub const ROTATE_RADAR_COUNTER_CLOCKWISE: CCommand = 0b1 << 8;
    pub const LOCK_GUN: CCommand = 0b1 << 9;
    pub const UNLOCK_GUN: CCommand = 0b1 << 10;
    pub const LOCK_RADAR: CCommand = 0b1 << 11;
    pub const UNLOCK_RADAR: CCommand = 0b1 << 12;
}