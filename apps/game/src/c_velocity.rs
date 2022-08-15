use bevy::prelude::*;
#[derive(Component)]
pub struct Velocity {
    pub velocity: f64,
}
#[derive(Component)]
pub struct TankVelocity {
    pub angular_velocity: f64,
    pub gun_angular_velocity: f64,
    pub radar_angular_velocity: f64,
}
