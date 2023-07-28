use std::process::Command as ProcessCommand;

use bevy::prelude::*;
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

pub struct DockerClient {
    pub tank_container_name: String,
}

impl ClientTrait for DockerClient {
    fn request_commands(&mut self) -> Vec<Command> {
        let output = ProcessCommand::new("bash")
            .arg("-c")
            .arg(format!(
                r#"curl {}:8080/request_commands | jq --raw-output '.[]'"#,
                self.tank_container_name,
            ))
            .output()
            .expect("failed to communicate with tank");

        if output.status.success() {
            let result_raw = String::from_utf8_lossy(&output.stdout);
            return parse_commands(result_raw.to_string());
        }

        let _err_raw = String::from_utf8_lossy(&output.stderr);
        println!(
            "SELF_DESTRUCT {:?} empty request_commands",
            self.tank_container_name
        );
        vec![Commands::SELF_DESTRUCT]
    }

    fn request_commands_by_event(&mut self, event: &CTEvent) -> Vec<Command> {
        let output = ProcessCommand::new("bash")
            .arg("-c")
            .arg(format!(
                r#"curl -d '{}' -X POST {}:8080/request_commands_by_event | jq --raw-output '.[]'"#,
                serde_json::to_string(event).unwrap(),
                self.tank_container_name,
            ))
            .output()
            .expect("failed to communicate with ocypod");

        if output.status.success() {
            let result_raw = String::from_utf8_lossy(&output.stdout);
            return parse_commands(result_raw.to_string());
        }
        let _err_raw = String::from_utf8_lossy(&output.stderr);
        vec![]
    }
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