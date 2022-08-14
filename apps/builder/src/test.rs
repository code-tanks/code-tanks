use std::{env, process::Command};

// use ctbuilder::build;

fn main() {
    // let args: Vec<String> = env::args().collect();

    // println!("url={}, lang={}", &args[1], &args[2]);

    // build(&args[1], &args[2]);

    let output_raw = Command::new("bash")
        .arg("-c")
        .arg(r#"echo {\"id\":1, \"input\":\"hello\"} | jq --raw-output '.id,.input'"#)
        .output()
        .expect("failed to communicate with ocypod");

    let result_raw = String::from_utf8_lossy(&output_raw.stdout);
    let err_raw = String::from_utf8_lossy(&output_raw.stderr);

    println!("stdout:");
    println!("{}", result_raw.to_string());
    println!("stderr:");
    println!("{}", err_raw.to_string());
    println!(
        "{}",
        &result_raw
            .to_string()
            .split('\n')
            .collect::<Vec<&str>>()
            .len()
    );
}
