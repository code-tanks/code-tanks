pub mod c_client;
// pub mod c_collider;
pub mod c_command;
pub mod c_event;
pub mod c_health;
// pub mod c_position;
pub mod c_render;
pub mod c_scanner;
pub mod c_tank;

// pub mod c_velocity;

pub mod s_apply_commands;
pub mod s_physics;
pub mod s_render;
pub mod s_request_commands;
pub mod s_request_commands_by_event;
pub mod s_save_commands;
pub mod s_setup_tanks;
pub mod s_walls;

use bevy::app::ScheduleRunnerSettings;

// use c_velocity::*;

use std::fs::File;
use std::io::Write;
use std::time::Duration;

use s_apply_commands::*;

use s_request_commands::*;
use s_request_commands_by_event::*;
use s_save_commands::*;
use s_setup_tanks::*;
use s_walls::*;

use bevy::prelude::*;

#[derive(Default)]
pub struct CState {
    pub tick: u32,
    pub tank_ids: Vec<String>,
}

pub fn run_game(tank_ids: &[String]) {
    let mut f = File::create("./sim.txt").expect("Unable to create file");
    f.write_all(format!("{}\n", tank_ids.join(",")).as_bytes())
        .expect("Unable to write data");

    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(MinimalPlugins)
        .insert_resource(CState {
            tick: 0,
            tank_ids: tank_ids.to_vec(),
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
            SystemStage::single_threaded().with_system(request_commands_by_event),
        )
        .run();
}
