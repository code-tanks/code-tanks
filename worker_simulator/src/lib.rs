use std::env;
use std::process::Command as ProcessCommand;

use ct_api::{Commands, Command};
use ctsimlib::c_client::{ClientTrait, parse_commands};
use ctsimlib::c_event::CTEvent;
use db::upload_log_to_db;
use postgres::Client;

pub mod db;
pub mod s_setup_sim_tanks;

pub fn create_sim_queue() {
    ProcessCommand::new("curl")
        .arg("-H")
        .arg("content-type: application/json")
        .arg("-XPUT")
        .arg("-d")
        .arg(r#"{"timeout": "10m"}"#)
        .arg(format!(
            "{}/queue/simulator",
            env::var("OCYPOD_URL").unwrap().parse::<String>().unwrap()
        ))
        .output()
        .expect("failed to communicate with ocypod");
}

pub fn get_sim_job() -> Vec<String> {
    let output_raw = ProcessCommand::new("bash")
        .arg("-c")
        .arg(format!(
            r#"curl {}/queue/simulator/job | jq --raw-output '.id,.input'"#,
            env::var("OCYPOD_URL").unwrap().parse::<String>().unwrap()
        ))
        .arg(format!(
            "{}/queue/build/job",
            env::var("OCYPOD_URL").unwrap().parse::<String>().unwrap()
        ))
        .output()
        .expect("failed to communicate with ocypod");

    let result_raw = String::from_utf8_lossy(&output_raw.stdout);

    result_raw
        .to_string()
        .split('\n')
        .map(|f| f.to_string())
        .filter(|f| !f.is_empty())
        .collect::<Vec<String>>()
}

pub fn upload_log(tank_container_name: &str, client: &mut Client) {
    let output_raw = ProcessCommand::new("docker")
        .arg("logs")
        .arg("--timestamps")
        .arg(tank_container_name)
        .output()
        .expect("failed to communicate with docker");
    let result_raw = String::from_utf8_lossy(&output_raw.stdout);
    let err_raw = String::from_utf8_lossy(&output_raw.stderr);

    upload_log_to_db(client, tank_container_name, &result_raw, &err_raw);
}

pub fn update_sim_job(id: &str, successful: bool) {
    ProcessCommand::new("curl")
        .arg("-H")
        .arg("content-type: application/json")
        .arg("-XPATCH")
        .arg("-d")
        .arg(format!(
            r#"{{"status": "{}"}}"#,
            if successful { "completed" } else { "failed" }
        ))
        .arg(format!(
            "{}/job/{}",
            env::var("OCYPOD_URL").unwrap().parse::<String>().unwrap(),
            id
        ))
        .output()
        .expect("failed to communicate with ocypod");

    println!("update job, id={}", id);
}

pub struct DockerClient {
    pub tank_container_name: String,
}

impl ClientTrait for DockerClient {
    fn request_commands(&mut self) -> Vec<Command> {
        let output = ProcessCommand::new("bash")
            .arg("-c")
            .arg(format!(
                r#"curl -sS -m 3 {}:8080/request_commands | jq --raw-output '.[]'"#,
                self.tank_container_name,
            ))
            .output()
            .expect("failed to communicate with tank");
        let err_raw = String::from_utf8_lossy(&output.stderr);

        if err_raw.is_empty() {
            let result_raw = String::from_utf8_lossy(&output.stdout);
            return parse_commands(result_raw.to_string());
        }

        // let _err_raw = String::from_utf8_lossy(&output.stderr);
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
                r#"curl -sS -m 3 -d '{}' -X POST {}:8080/request_commands_by_event | jq --raw-output '.[]'"#,
                serde_json::to_string(event).unwrap(),
                self.tank_container_name,
            ))
            .output()
            .expect("failed to communicate with ocypod");
        let err_raw = String::from_utf8_lossy(&output.stderr);

        if err_raw.is_empty() {
            let result_raw = String::from_utf8_lossy(&output.stdout);
            return parse_commands(result_raw.to_string());
        }
        // let _err_raw = String::from_utf8_lossy(&output.stderr);
        vec![]
    }
}
