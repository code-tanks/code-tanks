use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillMode, GeometryBuilder},
    shapes,
};
use ctsimlib::c_tank::Tank;

use crate::{
    c_particle::Particle,
    c_tracks::{Track, Tracks},
};

pub fn create_track(left: bool) -> shapes::Polygon {
    if left {
        shapes::Polygon {
            points: vec![
                Vec2::new(-15., -5.),
                Vec2::new(-15., 5.),
                Vec2::new(-10., 5.),
                Vec2::new(-10., -5.),
            ],
            closed: true,
        }
    } else {
        shapes::Polygon {
            points: vec![
                Vec2::new(15., -5.),
                Vec2::new(15., 5.),
                Vec2::new(10., 5.),
                Vec2::new(10., -5.),
            ],
            closed: true,
        }
    }
}

pub fn spawn_tracks(
    mut commands: Commands,
    mut query: Query<(&Transform, &mut Tracks), With<Tank>>,
) {
    for (transform, mut tracks) in &mut query {
        tracks.current_distant += transform.translation.distance(tracks.last_pos.translation);

        if tracks.current_distant > Tracks::MAX_DISTANCE {
            tracks.current_distant = 0.;

            let mut t = *transform;
            t.translation.z = 0.5;

            commands.spawn((
                Track { left: true },
                Particle {
                    progress: 0,
                    max_life_in_ticks: Track::MAX_LIFE_IN_TICKS,
                },
                GeometryBuilder::build_as(
                    &create_track(true),
                    DrawMode::Fill(FillMode::color(Color::rgba(0., 0., 0., Track::OPACITY))),
                    t,
                ),
            ));
            commands.spawn((
                Track { left: false },
                Particle {
                    progress: 0,
                    max_life_in_ticks: Track::MAX_LIFE_IN_TICKS,
                },
                GeometryBuilder::build_as(
                    &create_track(false),
                    DrawMode::Fill(FillMode::color(Color::rgba(0., 0., 0., Track::OPACITY))),
                    t,
                ),
            ));
        }

        tracks.last_pos = *transform;
    }
}
