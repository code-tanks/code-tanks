use bevy::prelude::*;
use std::f32::consts::PI;

#[derive(Component)]
pub struct Tank {
    pub info: TankInfo,
    pub cooldown: u32,
    pub gun: Entity,
    pub radar: Entity,
}
#[derive(Resource)]
pub struct AllTankInfo {
    pub all: Vec<TankInfo>,
}

#[derive(Clone)]
pub struct TankInfo {
    pub hash: String,
    pub id: String,
    pub index: usize,
    pub container_name: String,
}

impl Tank {
    pub const MAX_COOLDOWN: u32 = 60;
    pub const RADIUS: f32 = 19.0;
    pub const INITIAL_ROTATION: f32 = -PI/2.;
    pub const ROTATION_SPEED: f32 = PI * 0.3;
    pub const MOVEMENT_SPEED: f32 = 100.;
}

#[derive(Component)]
pub struct Bullet {
    pub tank: Entity,
}

impl Bullet {
    pub const RADIUS: f32 = 5.0;
    pub const SPEED: f32 = 200.0;
}

#[derive(Component)]
pub struct Radar {
    pub locked: bool,
}

#[derive(Component)]
pub struct Gun {
    pub locked: bool,
}

#[derive(Component)]
pub struct DamageDealer {
    pub damage_dealt: u32,
}
