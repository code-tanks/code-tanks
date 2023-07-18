use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
// use bevy_prototype_lyon::{
//     prelude::{GeometryBuilder, ShapeBundle, Fill},
//     shapes,
// };
use ctsimlib::c_tank::Tank;

use crate::{
    c_particle::Particle,
    c_tracks::{Track, Tracks},
};

// pub fn create_track(left: bool) -> MaterialMesh2dBundle {
//     if left {
//         shapes::Polygon {
//             points: vec![
//                 Vec2::new(-15., -5.),
//                 Vec2::new(-15., 5.),
//                 Vec2::new(-10., 5.),
//                 Vec2::new(-10., -5.),
//             ],
//             closed: true,
//         }
//     } else {
//         shapes::Polygon {
//             points: vec![
//                 Vec2::new(15., -5.),
//                 Vec2::new(15., 5.),
//                 Vec2::new(10., 5.),
//                 Vec2::new(10., -5.),
//             ],
//             closed: true,
//         }
//     }
// }

pub fn spawn_tracks(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut Tracks), With<Tank>>,
) {
    for (mut transform, mut tracks) in &mut query {
        tracks.current_distant += transform.translation.distance(tracks.last_pos.translation);

        if tracks.current_distant > Tracks::MAX_DISTANCE {
            // println!("!");
            tracks.current_distant = 0.;

            // let mut t = *transform;
            transform.translation.z = 1.;
            // t.translation.z = 0.5;

            let t = transform.rotation * Vec3::X;

            // println!("{:?}", t);

            commands.spawn((
                Track { left: true },
                Particle {
                    progress: 0,
                    max_life_in_ticks: Track::MAX_LIFE_IN_TICKS,
                },
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.25, 0.25, 0.75),
                        custom_size: Some(Vec2::new(5.0, 10.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(
                        transform.translation + t * Vec3::new(-10.0, -10.0, -10.0),
                    )
                    .with_rotation(transform.rotation),
                    // transform: Transform { translation: Vec3 { x: t.translation.x - 5., y: t.translation.y, z: t.translation.z}, ..default(), ..default() }.rotate_x(t.rotation.x),
                    // transform: t.with_translation(Vec3 { x: t.translation.x - 5., y: t.translation.y, z: t.translation.z}),
                    ..default()
                }, // ShapeBundle {
                   //     path: GeometryBuilder::build_as(&create_track(true)),
                   //     ..default()
                   // },
                   // Fill::color(Color::rgba(0., 0., 0., Track::OPACITY))
            ));
            commands.spawn((
                Track { left: false },
                Particle {
                    progress: 0,
                    max_life_in_ticks: Track::MAX_LIFE_IN_TICKS,
                },
                // ShapeBundle {
                //     path: GeometryBuilder::build_as(&create_track(false)),
                //     ..default()
                // },
                // Fill::color(Color::rgba(0., 0., 0., Track::OPACITY))
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.25, 0.25, 0.75),
                        custom_size: Some(Vec2::new(5.0, 10.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(
                        transform.translation + t * Vec3::new(10.0, 10.0, 10.0),
                    )
                    .with_rotation(transform.rotation),
                    ..default()
                },
            ));
        }

        tracks.last_pos = *transform;
    }
}
