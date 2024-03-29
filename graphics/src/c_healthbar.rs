use bevy::prelude::*;
use ctengine::c_tank::Tank;

#[derive(Component)]
pub struct HealthBar {
    pub tank: Entity,
    pub is_backdrop: bool,
}

impl HealthBar {
    pub const MAX_WIDTH: f32 = Tank::RADIUS * 2.0;
    pub const MAX_HEIGHT: f32 = 3.0;
}
