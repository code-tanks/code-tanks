use std::{process::Command, thread, time};

use bevy::{prelude::*, winit::WinitSettings};
use ctsimlib::{s_setup_walls::setup_walls, *};
use s_setup_desktop_tanks::setup_desktop_tanks;

pub mod s_setup_desktop_tanks;
use ctsimlib::core_plugin::CoreCTPlugin;
use ctsimlibgraphics::CoreCTGraphicsPlugin;

const PORTS: [usize; 4] = [8061, 8062, 8063, 8064];

pub fn run_local_tank(url: &str, game_url: &str, post_fix: usize, port: usize) -> String {
    let tank_id = format!("local-{}-{}-{}", game_url, url, post_fix);
    remove_tank(&tank_id);
    let output_raw = Command::new("docker")
        .arg("run")
        .arg("-d")
        .arg("-p")
        .arg(format!("{}:8080", port))
        .arg("--name")
        .arg(&tank_id)
        .arg("--label")
        .arg("code-tanks")
        .arg(format!("localhost:5001/{}", url))
        .output()
        .expect("failed to communicate with docker");
    let result_raw = String::from_utf8_lossy(&output_raw.stdout);

    println!("run stdout:");
    println!("{}", result_raw);
    tank_id
}

pub fn run_game(args: &[String]) {
    let game_url = args.join("");
    let tank_ids = args
        .iter()
        .enumerate()
        .map(|(i, url)| run_local_tank(url, &game_url, i, PORTS[i]))
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
