use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillMode, GeometryBuilder},
    shapes::{self, RectangleOrigin},
};
use ctsimlib::c_tank::Tank;

use crate::{c_particle::Particle, c_tracks::Tracks};

pub fn spawn_tracks(
    mut commands: Commands,
    mut query: Query<(&Transform, &mut Tracks), With<Tank>>,
) {
    for (transform, mut tracks) in &mut query {
        tracks.current_distant += transform.translation.distance(tracks.last_pos.translation);

        if tracks.current_distant > Tracks::MAX_DISTANCE {
            tracks.current_distant = 0.;

            commands.spawn((
                Particle {
                    progress: 0,
                    max_life_in_ticks: 20,
                },
                GeometryBuilder::build_as(
                    &shapes::Rectangle {
                        extents: Vec2::new(5.0, 10.0),
                        origin: RectangleOrigin::BottomLeft,
                    },
                    DrawMode::Fill(FillMode::color(Color::rgba(1., 0., 0., 1.))),
                    transform.clone(),
                ),
            ));
        }

        tracks.last_pos = transform.clone();
    }
}
