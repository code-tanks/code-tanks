use std::process::Command;

pub mod db;

enum Langs {}

impl Langs {
    const DART: &'static str = "dart";
}

pub fn get_lang(_url: &str) -> &'static str {
    Langs::DART
}

// pub fn get_queues() -> Vec<String> {
//     let output_raw = Command::new("curl")
//         .arg("mq:8023/queue")
//         .output()
//         .expect("failed to communicate with ocypod");

//     let result_raw = String::from_utf8_lossy(&output_raw.stdout);

//     serde_json::from_str(&result_raw.to_string()).unwrap()
// }

pub fn create_build_queue() {
    Command::new("curl")
        .arg("-H")
        .arg("content-type: application/json")
        .arg("-XPUT")
        .arg("-d")
        .arg(r#"{{"timeout": "10m"}}"#)
        .arg("mq:8023/queue/build")
        .output()
        .expect("failed to communicate with ocypod");
}

pub struct Job {
    pub id: u32,
    pub url: String,
}

pub fn get_job() -> Vec<String> {
    let output_raw = Command::new("bash")
        .arg("-c")
        .arg(r#"curl mq:8023/queue/build/job | jq --raw-output '.id,.input'"#)
        .arg("mq:8023/queue/build/job")
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

pub struct BuildInfo {
    pub successful: bool,
    pub log: String,
}

pub fn build(url: &str, lang: &str) -> BuildInfo {
    let output_raw = Command::new("docker")
        .arg("build")
        .arg("-t")
        .arg(format!("localhost:5001/{}", url))
        .arg("--network")
        .arg("host")
        .arg("--build-arg")
        .arg(format!("url={}", url))
        .arg("-f")
        .arg(format!("Dockerfiles/{}.Dockerfile", lang))
        .arg("Dockerfiles")
        .output()
        .expect("failed to communicate with docker");

    // docker build -t test --network host --build-arg url=ping -f dart.Dockerfile .

    let result_raw = String::from_utf8_lossy(&output_raw.stdout);
    let err_raw = String::from_utf8_lossy(&output_raw.stderr);

    // println!("out: {}", result_raw.to_string());
    // println!("err: {}", err_raw.to_string() != "");

    let successful = err_raw.to_string() == "";

    println!("build, url={}, successful={}", url, successful);
    println!("stdout:");
    println!("{}", result_raw.to_string());
    println!("");
    println!("stderr:");
    println!("{}", err_raw.to_string());
    println!("");

    if successful {
        return BuildInfo {
            successful: true,
            log: result_raw.to_string(),
        };
    }

    BuildInfo {
        successful: false,
        log: err_raw.to_string(),
    }
}

// pub fn simulate(urls: &[&str]) {
//     // docker network create --driver bridge FooAppNet
//     // docker run --rm --net=FooAppNet --name=component1 -p 9000:9000 component1-image
//     // docker run --rm --net=FooAppNet --name=component2 component2-image
// }

pub fn update_job(id: &str, successful: bool) {
    Command::new("curl")
        .arg("-H")
        .arg("content-type: application/json")
        .arg("-XPATCH")
        .arg("-d")
        .arg(format!(
            r#"{{"status": "{}"}}"#,
            if successful { "completed" } else { "failed" }
        ))
        .arg(format!("mq:8023/job/{}", id))
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

pub fn push_to_registry(url: &str) -> bool {
    let output_raw = Command::new("docker")
        .arg("push")
        .arg(format!("localhost:5001/{}", url))
        .output()
        .expect("failed to communicate with ocypod");

    let result_raw = String::from_utf8_lossy(&output_raw.stdout);
    let err_raw = String::from_utf8_lossy(&output_raw.stderr);

    // println!("out: {}", result_raw.to_string());
    // println!("err: {}", err_raw.to_string() != "");

    let successful = err_raw.to_string() == "";

    println!("push_to_registry, url={}, successful={}", url, successful);
    println!("stdout:");
    println!("{}", result_raw.to_string());
    println!("");
    println!("stderr:");
    println!("{}", err_raw.to_string());
    println!("");

    successful
}

pub fn remove_image(url: &str) -> bool {
    let output_raw = Command::new("docker")
        .arg("image")
        .arg("remove")
        .arg(format!("localhost:5001/{}", url))
        .output()
        .expect("failed to communicate with docker");

    let result_raw = String::from_utf8_lossy(&output_raw.stdout);
    let err_raw = String::from_utf8_lossy(&output_raw.stderr);

    let successful = err_raw.to_string() == "";

    println!("remove_image, url={}, successful={}", url, successful);
    println!("stdout:");
    println!("{}", result_raw.to_string());
    println!("");
    println!("stderr:");
    println!("{}", err_raw.to_string());
    println!("");

    successful
}
