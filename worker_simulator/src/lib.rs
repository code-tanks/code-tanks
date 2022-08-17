use std::process::Command;

pub mod db;

pub fn create_sim_queue() {
    Command::new("curl")
        .arg("-H")
        .arg("content-type: application/json")
        .arg("-XPUT")
        .arg("-d")
        .arg(r#"{{"timeout": "10m"}}"#)
        .arg("ocypod:8023/queue/simulation")
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
    let tank_id = format!("{}-{}-{}", game_url, url, post_fix);
    let _output_raw = Command::new("docker")
        .arg("run")
        .arg("-d")
        .arg("--name")
        .arg(&tank_id)
        .arg("--label")
        .arg("com.docker.compose.project=codetanks")
        .arg(format!("localhost:5001/{}", url))
        .output()
        .expect("failed to communicate with docker");

    tank_id
}
