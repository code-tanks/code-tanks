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
}

fn upload(path: &str, extension: &str) {
    let output_raw = std::process::Command::new("curl")
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
    // println!("");
    // println!("stderr:");
    // println!("{}", err_raw.to_string());
    // println!("");
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
            let extension = path.extension().unwrap().to_str().unwrap();

            upload(path_str, extension);

            // println!("upload {} with {} extension", path_str, extension);
        }
        Some(("logs", sub_matches)) => {
            println!(
                "logs for {}",
                sub_matches
                    .get_one::<String>("LOG_TARGET")
                    .expect("required")
            );
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}
