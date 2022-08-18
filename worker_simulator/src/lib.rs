use core::time;
use std::{process::Command, thread};

use ctsimlib::run_game;

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
    // let err_raw = String::from_utf8_lossy(&output_raw.stderr);

    // println!("stdout:");
    // println!("{}", result_raw.to_string());
    // println!("");
    // println!("stderr:");
    // println!("{}", err_raw.to_string());
    // println!("");
    result_raw
        .to_string()
        .split('\n')
        .map(|f| f.to_string())
        .filter(|f| !f.is_empty())
        .collect::<Vec<String>>()
}

pub fn run_tank(url: &str, game_url: &str, post_fix: usize) -> String {
    // docker run -d --network=codetanks_default -p  8080:8080 --name tank_id --label com.docker.compose.project=codetanks localhost:5001/url
    let tank_id = format!("{}-{}-{}", game_url, url, post_fix);
    let output_raw = Command::new("docker")
        .arg("run")
        .arg("-d")
        .arg("--network=codetanks_default")
        .arg("-p")
        .arg("8080:8080")
        .arg("--name")
        .arg(&tank_id)
        .arg("--label")
        .arg("com.docker.compose.project=codetanks")
        .arg(format!("localhost:5001/{}", url))
        .output()
        .expect("failed to communicate with docker");
    let result_raw = String::from_utf8_lossy(&output_raw.stdout);
    // let err_raw = String::from_utf8_lossy(&output_raw.stderr);

    println!("stdout:");
    println!("{}", result_raw.to_string());
    tank_id
}

pub fn remove_tank(tank_id: &str) {
    let output_raw = Command::new("docker")
        .arg("rm")
        .arg("-f")
        .arg(&tank_id)
        .output()
        .expect("failed to communicate with docker");
    let result_raw = String::from_utf8_lossy(&output_raw.stdout);
    // let err_raw = String::from_utf8_lossy(&output_raw.stderr);

    println!("stdout:");
    println!("{}", result_raw.to_string());
}

pub fn run_docker_game(args: &[String]) {
    let game_url = args.join("");
    let tank_ids = args
        .iter()
        .enumerate()
        .map(|(i, url)| run_tank(url, &game_url, i))
        .collect::<Vec<String>>();
    // for tank_id in tank_ids.iter() {
    //     remove_tank(tank_id);
    // }
    thread::sleep(time::Duration::from_millis(5000));

    run_game(&tank_ids);
    for tank_id in tank_ids.iter() {
        remove_tank(tank_id);
    }
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

    // let result_raw = String::from_utf8_lossy(&output_raw.stdout);
    // let err_raw = String::from_utf8_lossy(&output_raw.stderr);

    println!("update job, id={}", id);
    // println!("stdout:");
    // println!("{}", result_raw.to_string());
    // println!("");
    // println!("stderr:");
    // println!("{}", err_raw.to_string());
    // println!("");
}
