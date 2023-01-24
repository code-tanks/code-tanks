use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use ctsimlib::c_tank::Bullet;

pub fn on_added_bullet(mut commands: Commands, query: Query<Entity, Added<Bullet>>) {
    for e in query.iter() {
        commands.entity(e).with_children(|parent| {
            parent.spawn(GeometryBuilder::build_as(
                &shapes::Circle {
                    radius: Bullet::RADIUS,
                    center: Vec2::ZERO,
                },
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::GREEN),
                    outline_mode: StrokeMode::new(Color::BLACK, 1.0),
                },
                Transform::from_xyz(0., 0., 0.),
            ));
        });
    }
}
