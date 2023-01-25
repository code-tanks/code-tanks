use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Particle {
    pub progress: usize,
    pub max_life_in_ticks: usize,
}
