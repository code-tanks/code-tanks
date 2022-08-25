use std::process::Command;

use bevy::prelude::*;

use crate::{c_command::*, c_event::*};

#[derive(Component)]
pub struct Client {
    pub client: Box<dyn ClientTrait + Send + Sync>,
}

// impl Client {
//     // pub fn dummy() -> ClientConnection {
//     //     ClientConnection {
//     //         client: Box::new(DummyClient {}),
//     //     }
//     // }

//     // pub fn url(url: &str) -> Client {
//     //     Client {
//     //         client: Box::new(DummyClient {}),
//     //     }
//     // }

//     // pub fn reader(lines: Vec<String>) -> Client {
//     //     Client {
//     //         client: Box::new(ReaderClient { lines }),
//     //     }
//     // }
// }

pub trait ClientTrait {
    fn request_commands(&mut self) -> Vec<CCommand>;
    fn request_commands_by_event(&mut self, event: &Event) -> Vec<CCommand>;
}

pub struct DockerClient {
    pub tank_id: String,
}

impl ClientTrait for DockerClient {
    fn request_commands(&mut self) -> Vec<CCommand> {
        println!(
                r#"curl {}:8080/request_commands | jq --raw-output '.[]'"#,
                self.tank_id);
        let output_raw = Command::new("bash")
            .arg("-c")
            .arg(format!(
                r#"curl {}:8080/request_commands | jq --raw-output '.[]'"#,
                self.tank_id,
            ))
            // .arg("ocypod:8023/queue/build/job")
            .output()
            .expect("failed to communicate with ocypod");

        // let result_raw = String::from_utf8_lossy(&output_raw.stdout);
        // let err_raw = String::from_utf8_lossy(&output_raw.stderr);

        // let mut res: Vec<CCommand> = vec![];
        let result_raw = String::from_utf8_lossy(&output_raw.stdout);
        let err_raw = String::from_utf8_lossy(&output_raw.stderr);
    
        // println!("out: {}", result_raw.to_string());
        // println!("err: {}", err_raw.to_string() != "");
    
        let successful = err_raw.to_string() == "";
    
        println!("tank_id={}, successful={}", self.tank_id, successful);
        println!("stdout:");
        println!("{}", result_raw.to_string());
        println!("");
        println!("stderr:");
        println!("{}", err_raw.to_string());
        println!("");
    
        // if err_raw.to_string() == "" {
            // res = 
            result_raw
                .to_string()
                .split('\n')
                .map(|f| f.to_string())
                .filter(|f| !f.is_empty())
                .filter_map(|f| f.parse::<CCommand>().ok())
                .collect::<Vec<CCommand>>()
        //         ;
        // } else {
        //     println!("stderr:");
        //     println!("{}", err_raw.to_string());
        //     println!("");
        // }

        // res
    }

    fn request_commands_by_event(&mut self, _event: &Event) -> Vec<CCommand> {
        let output_raw = Command::new("bash")
            .arg("-c") 
            .arg(format!( 
                r#"curl -d {{"event_type": 0,"info":{{}}}} -X POST {}:8080/request_commands_by_event | jq --raw-output '.[]'"#,
                self.tank_id,  
            ))
            // .arg("ocypod:8023/queue/build/job")
            .output()
            .expect("failed to communicate with ocypod");

        let result_raw = String::from_utf8_lossy(&output_raw.stdout);
        // let err_raw = String::from_utf8_lossy(&output_raw.stderr);

        // let mut res: Vec<CCommand> = vec![];

        // if err_raw.to_string() == "" {
        //     res = 
            result_raw
                .to_string()
                .split('\n')
                .map(|f| f.to_string())
                .filter(|f| !f.is_empty())
                .filter_map(|f| f.parse::<CCommand>().ok())
                .collect::<Vec<CCommand>>()
        //         ;
        // } else {
        //     println!("stderr:");
        //     println!("{}", err_raw.to_string());
        //     println!("");
        // }

        // res
    }
}

pub struct DummyClient {}

impl ClientTrait for DummyClient {
    fn request_commands(&mut self) -> Vec<CCommand> {
        vec![CCommands::MOVE_FORWARD | CCommands::ROTATE_TANK_CLOCKWISE]
    }

    fn request_commands_by_event(&mut self, event: &Event) -> Vec<CCommand> {
        match event.event_type {
            EventType::Scan => {
                vec![CCommands::MOVE_FORWARD]
            }
            EventType::Hit => {
                vec![CCommands::MOVE_FORWARD]
            }
        }
    }
}

pub struct ReaderClient {
    pub lines: Vec<CCommand>,
}

impl ClientTrait for ReaderClient {
    fn request_commands(&mut self) -> Vec<CCommand> {
        if self.lines.is_empty() {
            vec![CCommands::NONE]
        } else {
            vec![self.lines.remove(0)]
        }
    }

    fn request_commands_by_event(&mut self, _event: &Event) -> Vec<CCommand> {
        if self.lines.is_empty() {
            vec![CCommands::NONE]
        } else {
            vec![self.lines.remove(0)]
        }
    }
}
