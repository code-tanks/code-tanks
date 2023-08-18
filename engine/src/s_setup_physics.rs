use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::TickState;

pub fn setup_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
    // TimestepMode::Variable {
    //     max_dt: TickState::SERVER_TICK_RATE as f32,
    //     time_scale: 1.0,
    //     substeps: 1,
    // }
    rapier_config.timestep_mode = TimestepMode::Fixed {
        dt: TickState::DT,
        substeps: 1,
    };
}
