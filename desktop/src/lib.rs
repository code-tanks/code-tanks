use std::{process::Command, thread, time};

use bevy::{prelude::*, winit::WinitSettings};
use ctsimlib::{s_setup_walls::setup_walls, *};
use s_setup_desktop_tanks::setup_desktop_tanks;

pub mod s_setup_desktop_tanks;
use ctsimlib::core_plugin::CoreCTPlugin;
use ctsimlibgraphics::CoreCTGraphicsPlugin;

pub fn remove_tank(tank_id: &str) {
    Command::new("docker")
    .arg("rm")
    .arg("--force")
    .arg(&tank_id)
    .output()
    .expect("failed to communicate with docker");
}

pub fn run_tank(url: &str, game_url: &str, post_fix: usize) -> String {
    // docker run -d --network=code-tanks_default -p  8080:8080 --name tank_id --label com.docker.compose.project=code-tanks localhost:5001/url
    let tank_id = format!("{}-{}-{}", game_url, url, post_fix);
    remove_tank(&tank_id);
    let output_raw = Command::new("docker")
        .arg("run")
        .arg("-d")
        .arg("--network=code-tanks_default")
        .arg("-p")
        .arg("8080")
        // .arg("-p")
        // .arg(format!("808{}:8080", post_fix))
        .arg("--name")
        .arg(&tank_id)
        .arg("--label")
        .arg("com.docker.compose.project=code-tanks")
        .arg(format!("localhost:5001/{}", url))
        .output()
        .expect("failed to communicate with docker");
    let result_raw = String::from_utf8_lossy(&output_raw.stdout);
    // let err_raw = String::from_utf8_lossy(&output_raw.stderr);

    println!("running tank_id {} on port 8080", tank_id);

    println!("run stdout:");
    println!("{}", result_raw.to_string());

    // let output_raw = Command::new("bash")
    //     .arg("-c")
    //     .arg(format!(r#"docker port {} 8080 | cut -d: -f2"#, tank_id))
    //     .output()
    //     .expect("failed to communicate with docker");
    // let result_raw = String::from_utf8_lossy(&output_raw.stdout);
    // // let err_raw = String::from_utf8_lossy(&output_raw.stderr);
    // let port = result_raw
    //     .to_string()
    //     .split("\n")
    //     .map(|f| f.to_string())
    //     .collect::<Vec<String>>();

    // let port = port.first().unwrap();

    // println!("port stdout:");
    // println!("{}", port);
    // format!("{}:{}", tank_id, port)
    tank_id
}

pub fn run_game(args: &[String]) {
    let game_url = args.join("");
    let tank_ids = args
        .iter()
        .enumerate()
        .map(|(i, url)| run_tank(url, &game_url, i))
        .collect::<Vec<String>>();

    thread::sleep(time::Duration::from_millis(1000));

    App::new()
        .insert_resource(WinitSettings {
            return_from_run: true,
            ..default()
        })
        .add_plugin(CoreCTPlugin)
        .add_plugin(CoreCTGraphicsPlugin)
        .insert_resource(TankIds {
            tank_ids: tank_ids.to_vec(),
        })
        .add_system(setup_desktop_tanks)
        .add_startup_system(setup_walls)
        .run();

    for tank_id in tank_ids {
        remove_tank(&tank_id);
        println!("removed {}", &tank_id);
    }

    println!("finished");
}
