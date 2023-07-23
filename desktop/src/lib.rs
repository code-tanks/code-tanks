use std::process::Command;

use bevy::{ecs::schedule::ScheduleLabel, prelude::*, winit::WinitSettings};
use ctsimlib::{s_setup_walls::setup_walls, *};
use s_setup_desktop_tanks::setup_desktop_tanks;
use bevy_rapier2d::prelude::RapierDebugRenderPlugin;

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
        .arg(url)
        .output()
        .expect("failed to communicate with docker");
    let result_raw = String::from_utf8_lossy(&output_raw.stdout);

    println!("run stdout:");
    println!("{}", result_raw);
    tank_id
}

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SetupDesktopTanks;

#[derive(Default, Resource)]
pub struct UseDummy {
    pub use_dummy: bool,
}

pub fn run_game(tank_ids: &[String]) {
    let game_url = tank_ids.join("");

    // thread::sleep(time::Duration::from_millis(1000));

    let tank_nametags = tank_ids
        .iter()
        .enumerate()
        .map(|(i, url)| run_local_tank(url, &game_url, i, PORTS[i]))
        .collect::<Vec<String>>();

    App::new()
        .insert_resource(WinitSettings {
            return_from_run: true,
            ..default()
        })
        .add_plugins(CoreCTPlugin)
        .add_plugins(CoreCTGraphicsPlugin)
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_desktop_tanks)
        .add_systems(Startup, setup_walls)
        .insert_resource(UseDummy {
            use_dummy: tank_ids.is_empty(),
        })
        .insert_resource(TankInfo {
            tank_ids: tank_ids.to_vec(),
            tank_nametags: tank_nametags.to_vec(),
        })
        .run();

    for tank_id in tank_nametags {
        remove_tank(&tank_id);
        println!("removed {}", &tank_id);
    }

    println!("finished");
}
