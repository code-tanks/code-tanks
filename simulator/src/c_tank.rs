use bevy::prelude::*;

#[derive(Component)]
pub struct Tank {
    pub cooldown: u32,
    pub gun: Entity,
    pub radar: Entity,
}

impl Tank {
    pub const MAX_COOLDOWN: u32 = 60;
    pub const WIDTH: f32 = 19.0;
    pub const HEIGHT: f32 = 23.0;
}

#[derive(Component)]
pub struct Bullet {
    pub tank: Entity,
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