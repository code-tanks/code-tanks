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

pub fn build(url: &str, lang: &str) {
    // Command::new("docker")
    //     .arg("build")
    //     .arg("-t")
    //     .arg(url)
    //     .arg("-f")
    //     .arg(format!("{}.Dockerfile", lang))
    //     .output()
    //     .expect("failed to communicate with docker");

    let output_raw = Command::new("docker")
        .arg("run")
        .arg("hello-world")
        .output()
        .expect("failed to communicate with docker");

    // docker build -t test --network host --build-arg url=ping -f dart.Dockerfile .

    let result_raw = String::from_utf8_lossy(&output_raw.stdout);

    println!("{}", result_raw.to_string());
}
