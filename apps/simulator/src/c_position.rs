use bevy_ecs::prelude::*;
#[derive(Component, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}
