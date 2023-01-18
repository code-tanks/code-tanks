use bevy::prelude::{Color, Query, Transform, Vec2, Without};
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillMode, Path, ShapePath, StrokeMode},
    shapes::{self, RectangleOrigin},
};
use ctsimlib::{c_health::Health, c_tank::Tank};

use crate::c_healthbar::HealthBar;

pub fn update_healthbar(
    q_parent: Query<(&Health, &Transform)>,
    mut q: Query<(&mut Path, &mut DrawMode, &mut Transform, &HealthBar), Without<Health>>,
) {
    for (mut path, mut draw_mode, mut transform, healthbar) in &mut q {
        let (health, p_transform) = q_parent.get(healthbar.tank).unwrap();

        let polygon = shapes::Rectangle {
            extents: Vec2::new(
                {
                    if healthbar.is_backdrop {
                        HealthBar::MAX_WIDTH
                    } else {
                        HealthBar::MAX_WIDTH * (health.val as f32) / (Health::MAX_HEALTH as f32)
                    }
                },
                HealthBar::MAX_HEIGHT,
            ),
            origin: RectangleOrigin::BottomLeft,
        };

        *path = ShapePath::build_as(&polygon);

        *draw_mode = DrawMode::Outlined {
            fill_mode: FillMode::color({
                if healthbar.is_backdrop {
                    Color::GRAY
                } else if (health.val as f32) <= (Health::MAX_HEALTH as f32) / 2.0 {
                    Color::RED
                } else {
                    Color::GREEN
                }
            }),
            outline_mode: StrokeMode::new(Color::BLACK, 1.0),
        };

        transform.translation.x = p_transform.translation.x - HealthBar::MAX_WIDTH / 2.0;
        transform.translation.y = p_transform.translation.y - Tank::RADIUS;
    }
}
