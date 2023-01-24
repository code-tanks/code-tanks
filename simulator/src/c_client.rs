use std::process::Command as ProcessCommand;

use bevy::prelude::*;
use ct_api::{Commands, Command};

use crate::c_event::*;

#[derive(Component)]
pub struct Client {
    pub client: Box<dyn ClientTrait + Send + Sync>,
}

pub trait ClientTrait {
    fn request_commands(&mut self) -> Vec<Command>;
    fn request_commands_by_event(&mut self, event: &Event) -> Vec<Command>;
}

pub struct DockerClient {
    pub tank_id: String,
}

impl ClientTrait for DockerClient {
    fn request_commands(&mut self) -> Vec<Command> {
        let output = ProcessCommand::new("bash")
            .arg("-c")
            .arg(format!(
                r#"curl {}:8080/request_commands | jq --raw-output '.[]'"#,
                self.tank_id,
            ))
            .output()
            .expect("failed to communicate with tank");

        let result_raw = String::from_utf8_lossy(&output.stdout);
        let _err_raw = String::from_utf8_lossy(&output.stderr);

        parse_commands(result_raw.to_string())
    }

    fn request_commands_by_event(&mut self, event: &Event) -> Vec<Command> {
        let output = ProcessCommand::new("bash")
            .arg("-c")
            .arg(format!(
                r#"curl -d '{}' -X POST {}:8080/request_commands_by_event | jq --raw-output '.[]'"#,
                serde_json::to_string(event).unwrap(),
                self.tank_id,
            ))
            .output()
            .expect("failed to communicate with ocypod");

        let result_raw = String::from_utf8_lossy(&output.stdout);
        let _err_raw = String::from_utf8_lossy(&output.stderr);

        parse_commands(result_raw.to_string())
    }
}

pub struct ReaderClient {
    pub lines: Vec<Command>,
}

impl ClientTrait for ReaderClient {
    fn request_commands(&mut self) -> Vec<Command> {
        if self.lines.is_empty() {
            vec![Commands::NONE]
        } else {
            vec![self.lines.remove(0)]
        }
    }

    fn request_commands_by_event(&mut self, _event: &Event) -> Vec<Command> {
        self.request_commands()
    }
}

fn parse_commands(commands_string: String) -> Vec<Command> {
    commands_string
        .split('\n')
        .map(|f| f.to_string())
        .filter(|f| !f.is_empty())
        .filter_map(|f| f.parse::<Command>().ok())
        .collect::<Vec<Command>>()
}
