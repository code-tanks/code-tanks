pub mod c_client;
pub mod c_command_source;
pub mod c_event;
pub mod c_health;
pub mod c_tank;
pub mod core_plugin;

pub mod s_apply_commands;
pub mod s_bullet_physics;
pub mod s_radar_physics;
pub mod s_request_commands;
pub mod s_request_commands_by_event;
pub mod s_save_commands;
pub mod s_setup_physics;
pub mod s_setup_sim_tanks;
pub mod s_setup_walls;
pub mod s_tank_physics;

use std::fs::File;
use std::io::Write;
use std::process::Command;

use bevy::app::App;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::{Component, IntoSystemConfigs, Resource, Startup, Update};
use bevy::MinimalPlugins;
use core_plugin::*;
use s_apply_commands::apply_commands;
use s_request_commands::request_commands;
use s_save_commands::*;
use s_setup_sim_tanks::*;
use s_setup_walls::*;

#[derive(Default, Resource)]
pub struct TickState {
    pub tick: u32,
}

impl TickState {
    pub const MAXIMUM_SIMULATION_TICKS: u32 = 300 * 2; // 10 secs
    pub const DT: f32 = 1.0 / 60.0;
}

// #[derive(Default, Resource)]
// pub struct TankInfo {
//     pub tank_ids: Vec<String>,
//     pub tank_container_name: Vec<String>,
// }

pub struct Game {}

impl Game {
    pub const WIDTH: f32 = 1024.;
    pub const HEIGHT: f32 = 640.0;
}

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SaveCommands;

// pub fn run_game(tank_ids: &[String], tank_container_names: &[String]) {
//     let mut f = File::create("./sim.txt").expect("Unable to create file");
//     f.write_all(format!("{}\n", tank_ids.join(",")).as_bytes())
//         .expect("Unable to write data");

//     App::new()
//         .add_plugins(MinimalPlugins)
//         .insert_resource(TankInfo {
//             tank_ids: tank_ids.to_vec(),
//             tank_container_name: tank_container_names.to_vec(),
//         })
//         .add_systems(Startup, (setup_walls, setup_sim_tanks).chain())
//         .add_plugins(CoreCTPlugin)
//         .add_systems(
//             Update,
//             save_commands.after(request_commands).before(apply_commands)
//             // "request_commands",
//             // "save_commands",
//             // SystemStage::single_threaded().with_system(save_commands),
//         )
//         .run();
// }

pub enum CollisionMask {}

impl CollisionMask {
    pub const NONE: u32 = 0b0;
    pub const TANK: u32 = 0b1;
    pub const WALL: u32 = 0b1 << 1;
    pub const BULLET: u32 = 0b1 << 2;
    pub const RADAR: u32 = 0b1 << 3;
}

#[derive(Component)]
pub struct CCollider {
    pub collision_type: CollisionType,
}

#[derive(Debug, PartialEq)]
pub enum CollisionType {
    Bullet,
    Tank,
    Wall,
    Radar,
}

pub fn remove_tank(tank_container_name: &str) {
    Command::new("docker")
        .arg("rm")
        .arg("--force")
        .arg(tank_container_name)
        .output()
        .expect("failed to communicate with docker");
}

// pub fn run_tank(url: &str, game_url: &str, post_fix: usize) {
// let tank_container_name = format!("{}-{}-{}", game_url, url, post_fix);

pub fn run_tank(tank_container_name: &str, tank_image_name: &str, port: &str, no_internet: bool) {
    remove_tank(&tank_container_name);
    let output_raw = Command::new("docker")
        .arg("run")
        .arg("-d")
        .arg(if no_internet {
            "--network=no-internet"
        } else {
            "--network=bridge"
        })
        // .arg("--network=code-tanks_no-internet")
        .arg("-p")
        .arg(port)
        .arg("--name")
        .arg(&tank_container_name)
        // .arg("--label")
        // .arg("com.docker.compose.project=code-tanks")
        .arg(tank_image_name)
        .output()
        .expect("failed to communicate with docker");
    let result_raw = String::from_utf8_lossy(&output_raw.stdout);

    println!("run stdout:");
    println!("{}", result_raw);
    // tank_container_name
}
