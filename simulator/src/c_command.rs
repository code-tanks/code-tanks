use bevy::prelude::*;
use ct_api::CCommand;

#[derive(Component)]
pub struct CommandSource {
    pub queue: Vec<CCommand>,
}

impl CommandSource {
    pub fn default() -> CommandSource {
        CommandSource { queue: Vec::new() }
    }
}
