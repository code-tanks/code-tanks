pub mod c_client;
// pub mod c_collider;
pub mod c_command;
pub mod c_event;
pub mod c_health;
// pub mod c_position;
pub mod c_render;
pub mod c_scanner;
pub mod c_tank;

pub mod db;
// pub mod c_velocity;

pub mod s_apply_commands;
pub mod s_physics;
pub mod s_publish_events;
pub mod s_render;
pub mod s_request_commands;
pub mod s_save_commands;
pub mod s_setup_tanks;
pub mod s_walls;

use bevy::app::ScheduleRunnerSettings;

// use c_velocity::*;

use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::time::Duration;

use s_apply_commands::*;

use s_publish_events::*;
use s_request_commands::*;
use s_save_commands::*;
use s_setup_tanks::*;
use s_walls::*;

use bevy::prelude::*;

#[derive(Default)]
pub struct CState {
    pub tick: u32,
    pub tanks: Vec<String>,
}

pub fn run_game(args: &[String]) {
    let mut f = File::create("./sim.txt").expect("Unable to create file");
    f.write_all(format!("{}\n", args.len()).to_string().as_bytes())
        .expect("Unable to write data");

    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(MinimalPlugins)
        .insert_resource(CState {
            tick: 0,
            tanks: args.to_vec(),
        })
        .add_startup_system(setup_walls)
        .add_startup_system(setup_tanks)
        .add_stage(
            "request_commands",
            SystemStage::single_threaded().with_system(request_commands),
        )
        .add_stage(
            "save_commands",
            SystemStage::single_threaded().with_system(save_commands),
        )
        .add_stage(
            "apply_commands",
            SystemStage::single_threaded().with_system(apply_commands),
        )
        .add_stage(
            "publish_events",
            SystemStage::single_threaded().with_system(publish_events),
        )
        .run();
}

pub fn create_sim_queue() {
    Command::new("curl")
        .arg("-H")
        .arg("content-type: application/json")
        .arg("-XPUT")
        .arg("-d")
        .arg(r#"{{"timeout": "10m"}}"#)
        .arg("mq:8023/queue/simulation")
        .output()
        .expect("failed to communicate with ocypod");
}

pub fn get_job() -> Vec<String> {
    let output_raw = Command::new("bash")
        .arg("-c")
        .arg(r#"curl mq:8023/queue/simulator/job | jq --raw-output '.id,.input'"#)
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
