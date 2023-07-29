use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Tracks {
    pub current_distant: f32,
    pub last_pos: Transform,
}

impl Tracks {
    pub const MAX_DISTANCE: f32 = 20.0;
}

#[derive(Component, Default)]
pub struct Track {
    pub left: bool,
}
impl Track {
    pub const MAX_LIFE_IN_TICKS: usize = 400;
    pub const OPACITY: f32 = 0.5;
}
