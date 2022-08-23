use bevy::prelude::*;

#[derive(Component)]
pub struct Tank {
    pub cooldown: u32,
}

impl Tank {
    pub const MAX_COOLDOWN: u32 = 60;
}

#[derive(Component)]
pub struct Bullet {}
