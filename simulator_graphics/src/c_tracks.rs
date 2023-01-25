use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Tracks {
    pub current_distant: f32,
    pub last_pos: Transform,
}

impl Tracks {
    pub const MAX_DISTANCE: f32 = 100.0;
}


