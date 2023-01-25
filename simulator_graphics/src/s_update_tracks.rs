use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{DrawMode, FillMode};

use crate::{c_particle::Particle, c_tracks::Track};

pub fn update_tracks(
    mut commands: Commands,

    mut query: Query<(Entity, &mut DrawMode, &mut Particle), With<Track>>,
) {
    for (entity, mut draw_mode, mut particle) in &mut query {
        if particle.progress > particle.max_life_in_ticks {
            commands.entity(entity).despawn_recursive();
        }

        let opacity = Track::OPACITY
            * (1.0 - (particle.progress as f32 / particle.max_life_in_ticks as f32).powf(3.));

        *draw_mode = DrawMode::Fill(FillMode::color(Color::rgba(0., 0., 0., opacity)));

        particle.progress += 1;
    }
}
