use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{DrawMode, FillMode, Path};

use crate::{c_particle::Particle, c_tracks::Track};

pub fn update_tracks(
    mut commands: Commands,

    mut query: Query<(Entity, &mut Path, &mut DrawMode, &Transform, &mut Particle, &Track)>,
) {
    for (entity, mut path, mut draw_mode, transform, mut particle, track) in &mut query {
        if particle.progress > particle.max_life_in_ticks {
            commands.entity(entity).despawn_recursive();
        }
        
        let opacity = 1.0 - (particle.progress as f32 / particle.max_life_in_ticks as f32).powf(3.);

        *draw_mode = DrawMode::Fill(FillMode::color(Color::rgba(1., 0., 0., opacity)));

        particle.progress += 1;
    }
}
