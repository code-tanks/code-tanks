pub mod c_client;
pub mod c_command;
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

use std::process::Command;

use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::{Component, Resource};

#[derive(Default, Resource)]
pub struct TickState {
    pub tick: u32,
}

impl TickState {
    pub const MAXIMUM_SIMULATION_TICKS: u32 = 300 * 2; // 10 secs
    pub const DT: f32 = 1.0 / 60.0;
}

#[derive(Default, Resource)]
pub struct TankInfo {
    pub tank_ids: Vec<String>,
    pub tank_nametags: Vec<String>,
}

pub struct Game {}

impl Game {
    pub const WIDTH: f32 = 1024.;
    pub const HEIGHT: f32 = 640.0;
}

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SaveCommands;




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

pub fn remove_tank(tank_id: &str) {
    Command::new("docker")
        .arg("rm")
        .arg("--force")
        .arg(tank_id)
        .output()
        .expect("failed to communicate with docker");
}