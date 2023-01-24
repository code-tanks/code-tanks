use bevy::prelude::*;
use ct_api::Command;

#[derive(Component, Default)]
pub struct CommandSource {
    pub queue: Vec<Command>,
}
