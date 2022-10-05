use std::path::PathBuf;

use clap::{arg, Command};
use colored::*;

fn cli() -> Command {
    Command::new("ctcli")
        .about("Code Tanks CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("logs")
                .about("Get logs")
                .arg(arg!(<LOG_TARGET> "The run/build id to get logs"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("upload")
                .about("Upload tank code")
                .arg_required_else_help(true)
                .arg(arg!(<PATH> "Path to tank file").value_parser(clap::value_parser!(PathBuf))),
        )
        .subcommand(
            Command::new("raw")
                .about("Download tank code")
                .arg_required_else_help(true)
                .arg(arg!(<TANK_ID> "The tank id to download")),
        )
        .subcommand(
            Command::new("run")
                .about("Run simulation with tank ids")
                .arg_required_else_help(true)
                .arg(arg!(<TANK_ID> ... "The tank ids to run")),
        )
}

fn upload(path: &str, extension: &str) {
    let output_raw = std::process::Command::new("curl")
        .arg("-s")
        .arg("--data-binary")
        .arg(format!("@{}", path))
        .arg("-H")
        .arg("Content-Type: text/plain")
        .arg("-X")
        .arg("POST")
        .arg(format!("http://localhost:8089/upload/{}", extension))
        .output()
        .expect("failed to communicate with CodeTanks server");

    let result_raw = String::from_utf8_lossy(&output_raw.stdout).to_string();
    let err_raw = String::from_utf8_lossy(&output_raw.stderr).to_string();

    if err_raw != "" {
        println!("{}", err_raw);
    }
    if result_raw != "" {
        println!("Tank Id: {}", result_raw.to_string().green());
    }
}

fn get_logs(tank_id: &str) {
    let output_raw = std::process::Command::new("curl")
        .arg("-s")
        .arg(format!("http://localhost:8089/log/{}", tank_id))
        .output()
        .expect("failed to communicate with CodeTanks server");

    let result_raw = String::from_utf8_lossy(&output_raw.stdout).to_string();
    let err_raw = String::from_utf8_lossy(&output_raw.stderr).to_string();

    if err_raw != "" {
        println!("{}", err_raw);
    }
    if result_raw != "" {
        println!("{}", result_raw);
    }
}

fn get_raw(tank_id: &str) {
    let output_raw = std::process::Command::new("curl")
        .arg("-s")
        .arg(format!("http://localhost:8089/raw/{}", tank_id))
        .output()
        .expect("failed to communicate with CodeTanks server");

    let result_raw = String::from_utf8_lossy(&output_raw.stdout).to_string();
    let err_raw = String::from_utf8_lossy(&output_raw.stderr).to_string();

    if err_raw != "" {
        println!("{}", err_raw);
    }
    if result_raw != "" {
        println!("{}", result_raw);
    }
}

fn run_sim(tank_ids: Vec<String>) {
    let output_raw = std::process::Command::new("curl")
        .arg("-s")
        .arg("-d")
        .arg(format!("{}", tank_ids.join(" ")))
        .arg("-X")
        .arg("POST")
        .arg("http://localhost:8089/run")
        .output()
        .expect("failed to communicate with CodeTanks server");

    let result_raw = String::from_utf8_lossy(&output_raw.stdout).to_string();
    let err_raw = String::from_utf8_lossy(&output_raw.stderr).to_string();

    if err_raw != "" {
        println!("{}", err_raw);
    }
    if result_raw != "" {
        println!("{}", result_raw);
    }
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("upload", sub_matches)) => {
            let path = sub_matches
                .get_one::<PathBuf>("PATH")
                .expect("required")
                .as_path();
            let path_str = path.to_str().unwrap();

            if !path.exists() {
                println!("Path '{}' does not exist.", path_str.red());
            } else if path.is_dir() {
                println!(
                    "Path '{}' is a directory. Your upload must be a file.",
                    path_str.red()
                );
            } else {
                let extension = path.extension().unwrap().to_str().unwrap();

                upload(path_str, extension);
            }
        }
        Some(("logs", sub_matches)) => {
            get_logs(
                sub_matches
                    .get_one::<String>("LOG_TARGET")
                    .expect("required"),
            );
        }
        Some(("raw", sub_matches)) => {
            get_raw(sub_matches.get_one::<String>("TANK_ID").expect("required"));
        }
        Some(("run", sub_matches)) => {
            run_sim(
                sub_matches
                    .get_many::<String>("TANK_ID")
                    .expect("required")
                    .map(|f| f.to_string())
                    .collect(),
            );
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}
