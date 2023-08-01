
use bevy::prelude::Component;
// use bevy::prelude::*;
use ct_api::{Command, Commands};

use crate::c_event::*;

#[derive(Component)]
pub struct Client {
    pub client: Box<dyn ClientTrait + Send + Sync>,
}

pub trait ClientTrait {
    fn request_commands(&mut self) -> Vec<Command>;
    fn request_commands_by_event(&mut self, event: &CTEvent) -> Vec<Command>;
}

pub fn parse_commands(commands_string: String) -> Vec<Command> {
    // println!("parsing commands {}", commands_string);

    commands_string
        .split('\n')
        .map(|f| f.to_string())
        .filter(|f| !f.is_empty())
        .filter_map(|f| f.parse::<Command>().ok())
        .collect::<Vec<Command>>()
}