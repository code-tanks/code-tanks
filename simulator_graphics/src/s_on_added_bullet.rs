use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use ctsimlib::c_tank::{Bullet, Tank};

const COLORS: &[Color] = &[
    Color::RED,
    Color::GREEN,
    Color::BLUE,
    Color::GRAY,
];

pub fn on_added_bullet(
    mut commands: Commands,
    query_tank: Query<&Tank>,
    query_bullet: Query<&Bullet>,
    query: Query<Entity, Added<Bullet>>,
) {
    for e in query.iter() {
        commands.entity(e).with_children(|parent| {
            parent.spawn(GeometryBuilder::build_as(
                &shapes::Circle {
                    radius: Bullet::RADIUS,
                    center: Vec2::ZERO,
                },
                DrawMode::Outlined {
                    fill_mode: FillMode::color(
                        COLORS[query_tank
                            .get(query_bullet.get(e).unwrap().tank)
                            .unwrap()
                            .index
                            % COLORS.len()],
                    ),
                    outline_mode: StrokeMode::new(Color::BLACK, 1.0),
                },
                Transform::from_xyz(0., 0., 0.),
            ));
        });
    }
}
