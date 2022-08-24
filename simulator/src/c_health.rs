use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub val: i32,
}

impl Health {
    pub const MAX_HEALTH: i32 = 100;
}
