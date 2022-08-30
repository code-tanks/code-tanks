use bevy::prelude::*;

#[derive(Component)]
pub struct Tank {
    pub cooldown: u32,
    pub gun: Entity,
    pub radar: Entity,
}

impl Tank {
    pub const MAX_COOLDOWN: u32 = 60;
}

#[derive(Component)]
pub struct Bullet {}

#[derive(Component)]
pub struct Radar {
    pub locked: bool,
}

#[derive(Component)]
pub struct Gun {
    pub locked: bool,
}
