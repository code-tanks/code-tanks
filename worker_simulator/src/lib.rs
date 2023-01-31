use core::time;
use std::{process::Command, thread};

use ctsimlib::{run_game, run_tank};
use db::upload_log_to_db;
use postgres::Client;

pub mod db;

pub fn create_sim_queue() {
    Command::new("curl")
        .arg("-H")
        .arg("content-type: application/json")
        .arg("-XPUT")
        .arg("-d")
        .arg(r#"{"timeout": "10m"}"#)
        .arg("ocypod:8023/queue/simulator")
        .output()
        .expect("failed to communicate with ocypod");
}

pub fn get_sim_job() -> Vec<String> {
    let output_raw = Command::new("bash")
        .arg("-c")
        .arg(r#"curl ocypod:8023/queue/simulator/job | jq --raw-output '.id,.input'"#)
        .arg("ocypod:8023/queue/build/job")
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

pub fn upload_log(tank_id: &str, client: &mut Client) {
    let output_raw = Command::new("docker")
        .arg("logs")
        .arg("--timestamps")
        .arg(tank_id)
        .output()
        .expect("failed to communicate with docker");
    let result_raw = String::from_utf8_lossy(&output_raw.stdout);
    let err_raw = String::from_utf8_lossy(&output_raw.stderr);

    upload_log_to_db(
        client,
        tank_id,
        &result_raw,
        &err_raw,
    );
}

pub fn run_docker_game(args: &[String]) -> Vec<String> {
    let game_url = args.join("");
    let tank_ids = args
        .iter()
        .enumerate()
        .map(|(i, url)| run_tank(url, &game_url, i, false))
        .collect::<Vec<String>>();
    thread::sleep(time::Duration::from_millis(5000));

    run_game(&tank_ids);
    tank_ids
}

pub fn update_sim_job(id: &str, successful: bool) {
    Command::new("curl")
        .arg("-H")
        .arg("content-type: application/json")
        .arg("-XPATCH")
        .arg("-d")
        .arg(format!(
            r#"{{"status": "{}"}}"#,
            if successful { "completed" } else { "failed" }
        ))
        .arg(format!("ocypod:8023/job/{}", id))
        .output()
        .expect("failed to communicate with ocypod");

    println!("update job, id={}", id);
}
