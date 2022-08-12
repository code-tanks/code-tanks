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
        .arg("build")
        .arg("-t")
        .arg(url)
        .arg("--network")
        .arg("host")
        .arg("--build-arg")
        .arg(format!("url={}", url))
        .arg("-f")
        .arg(format!("{}.Dockerfile", lang))
        .arg(".")
        .output()
        .expect("failed to communicate with docker");

    // docker build -t test --network host --build-arg url=ping -f dart.Dockerfile .

    let result_raw = String::from_utf8_lossy(&output_raw.stdout);
    let err_raw = String::from_utf8_lossy(&output_raw.stderr);

    println!("out: {}", result_raw.to_string());
    println!("err: {}", err_raw.to_string());
}

pub fn simulate(urls: &[&str]) {
    // docker network create --driver bridge FooAppNet
    // docker run --rm --net=FooAppNet --name=component1 -p 9000:9000 component1-image
    // docker run --rm --net=FooAppNet --name=component2 component2-image
}
