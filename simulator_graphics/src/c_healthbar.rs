use bevy::prelude::*;
use ctsimlib::c_tank::Tank;

#[derive(Component)]
pub struct HealthBar {
    pub tank: Entity,
    pub is_backdrop: bool
}

impl HealthBar {
    pub const MAX_WIDTH: f32 = Tank::WIDTH * 2.0;
    pub const MAX_HEIGHT: f32 = 3.0;
}