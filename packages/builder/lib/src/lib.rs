use std::process::Command;

use serde_json::Value;

pub fn get_queues() -> Vec<String> {
    let output_raw = Command::new("curl")
        .arg("mq:8023/queue")
        .output()
        .expect("failed to communicate with ocypod");

    let result_raw = String::from_utf8_lossy(&output_raw.stdout);

    serde_json::from_str(&result_raw.to_string()).unwrap()
}

pub fn create_build_queue() {
    Command::new("curl")
        .arg("-H")
        .arg("content-type: application/json")
        .arg("-XPUT")
        .arg("-d")
        .arg(
            serde_json::json!({
                "timeout": "10m",
            })
            .to_string(),
        )
        .arg("mq:8023/queue/build")
        .output()
        .expect("failed to communicate with ocypod");
}

pub fn get_job() -> Result<Value, serde_json::Error> {
    let output_raw = Command::new("curl")
        .arg("mq:8023/queue/build/job")
        .output()
        .expect("failed to communicate with ocypod");

    let result_raw = String::from_utf8_lossy(&output_raw.stdout);

    serde_json::from_str(&result_raw.to_string())
}
