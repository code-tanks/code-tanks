use bevy_ecs::prelude::*;
#[derive(Component)]
pub struct Velocity {
    pub velocity: f32,
}
#[derive(Component)]
pub struct TankVelocity {
    pub angular_velocity: f32,
    pub gun_angular_velocity: f32,
    pub radar_angular_velocity: f32,
}
