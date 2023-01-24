use bevy::prelude::*;
use ct_api::Command;

#[derive(Component)]
pub struct CommandSource {
    pub queue: Vec<Command>,
}

impl CommandSource {
    pub fn default() -> CommandSource {
        CommandSource { queue: Vec::new() }
    }
}
