use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::TickState;

pub fn setup_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
    rapier_config.timestep_mode = TimestepMode::Fixed {
        dt: TickState::SERVER_TICK_RATE as f32,
        substeps: 1,
    };
}
