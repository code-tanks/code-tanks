pub mod c_client;
pub mod c_command;
pub mod c_event;
pub mod c_health;
pub mod c_tank;
pub mod core_plugin;

pub mod s_apply_commands;
pub mod s_physics;
pub mod s_request_commands;
pub mod s_request_commands_by_event;
pub mod s_request_debug_commands;
pub mod s_save_commands;
pub mod s_setup_sim_tanks;
pub mod s_setup_walls;

use bevy::app::ScheduleRunnerSettings;

use std::fs::File;
use std::io::Write;
use std::time::Duration;

use bevy::app::App;
use bevy::ecs::schedule::SystemStage;
use bevy::prelude::{Component, Resource};
use bevy::MinimalPlugins;
use core_plugin::*;
use s_save_commands::*;
use s_setup_sim_tanks::*;
use s_setup_walls::*;

#[derive(Default, Resource)]
pub struct TickState {
    pub tick: u32,
}

impl TickState {
    pub const MAXIMUM_SIMULATION_TICKS: u32 = 300;
    pub const TICK_RATE: f64 = 1.0 / 60.0;
}


#[derive(Default, Resource)]
pub struct TankIds {
    pub tank_ids: Vec<String>,
}

pub fn run_game(tank_ids: &[String]) {
    let mut f = File::create("./sim.txt").expect("Unable to create file");
    f.write_all(format!("{}\n", tank_ids.join(",")).as_bytes())
        .expect("Unable to write data");

    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            TickState::TICK_RATE,
        )))
        .add_plugins(MinimalPlugins)
        .insert_resource(TankIds {
            tank_ids: tank_ids.to_vec(),
        })
        .add_startup_system(setup_walls)
        .add_startup_system(setup_sim_tanks)
        .add_plugin(CoreCTPlugin)
        .add_stage_after(
            "request_commands",
            "save_commands",
            SystemStage::single_threaded().with_system(save_commands),
        )
        .run();
}

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
