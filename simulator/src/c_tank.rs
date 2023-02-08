use bevy::prelude::*;

#[derive(Component)]
pub struct Tank {
    pub id: String,
    pub index: usize,
    pub cooldown: u32,
    pub gun: Entity,
    pub radar: Entity,
}

impl Tank {
    pub const MAX_COOLDOWN: u32 = 60;
    pub const RADIUS: f32 = 19.0;
    pub const INITIAL_ROTATION: f32 = -90.0;
}

#[derive(Component)]
pub struct Bullet {
    pub tank: Entity,
}

impl Bullet {
    pub const RADIUS: f32 = 5.0;
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
