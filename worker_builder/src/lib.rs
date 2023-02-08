use std::process::Command;

pub mod db;

// enum Langs {}

// impl Langs {
//     const DART: &str = "dart";
// }

// pub fn get_lang(_url: &str) -> &str {
//     Langs::DART
// }

pub fn create_build_queue() {
    Command::new("curl")
        .arg("-H")
        .arg("content-type: application/json")
        .arg("-XPUT")
        .arg("-d")
        .arg(r#"{"timeout": "10m"}"#)
        .arg("ocypod:8023/queue/build")
        .output()
        .expect("failed to communicate with ocypod");
}

pub struct Job {
    pub id: u32,
    pub url: String,
}

pub fn get_build_job() -> Vec<String> {
    let output_raw = Command::new("bash")
        .arg("-c")
        .arg(r#"curl ocypod:8023/queue/build/job | jq --raw-output '.id,.input'"#)
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

pub fn build(url: &str, lang: &str) -> String {
    let output = Command::new("docker")
        .arg("build")
        .arg("-t")
        .arg(format!("registry:5001/{}", url))
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

    let result_raw = String::from_utf8_lossy(&output.stdout);
    let err_raw = String::from_utf8_lossy(&output.stderr);

    println!("build, url={}", url);
    println!("stdout:");
    println!("{}\n", result_raw);
    println!("stderr:");
    println!("{}\n", err_raw);

    format!("{}\n{}", result_raw, err_raw)
}

pub fn update_build_job(id: &str, successful: bool) {
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

pub fn push_to_registry(url: &str) -> bool {
    let output = Command::new("docker")
        .arg("push")
        .arg(format!("registry:5001/{}", url))
        .output()
        .expect("failed to communicate with ocypod");

    let result_raw = String::from_utf8_lossy(&output.stdout);
    let err_raw = String::from_utf8_lossy(&output.stderr);

    let successful = output.status.success();

    println!("push_to_registry, url={}, successful={}", url, successful);
    if successful {
    println!("stdout:");
    println!("{}\n", result_raw);
    } else {
    println!("stderr:");
    println!("{}\n", err_raw);
    }
    successful
}

pub fn remove_image(url: &str) -> bool {
    let output = Command::new("docker")
        .arg("image")
        .arg("remove")
        .arg(format!("registry:5001/{}", url))
        .output()
        .expect("failed to communicate with docker");

    let result_raw = String::from_utf8_lossy(&output.stdout);
    let err_raw = String::from_utf8_lossy(&output.stderr);

    let successful = output.status.success();

    println!("remove_image, url={}, successful={}", url, successful);
    if successful {
    println!("stdout:");
    println!("{}\n", result_raw);
    } else {
    println!("stderr:");
    println!("{}\n", err_raw);
    }

    successful
}
