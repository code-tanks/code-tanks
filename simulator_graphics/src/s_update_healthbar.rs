use bevy::prelude::{Color, Gizmos, Query, Transform, Vec2, With, ResMut, GizmoConfig};
use ctsimlib::{c_health::Health, c_tank::Tank};

use crate::c_healthbar::HealthBar;

pub fn update_healthbar(
    mut gizmos: Gizmos,
    q: Query<(&Transform, &Health), With<Tank>>,
    mut config: ResMut<GizmoConfig>,
) {
    config.line_width = 5.0;
    for (p_transform, health) in &q {
        // let (health, p_transform) = q_parent.get(healthbar.tank).unwrap();

        // let polygon = shapes::Rectangle {
        //     extents: Vec2::new(
        //         {
        //             if healthbar.is_backdrop {
        //                 HealthBar::MAX_WIDTH
        //             } else {
        //                 HealthBar::MAX_WIDTH * (health.val as f32) / (Health::MAX_HEALTH as f32)
        //             }
        //         },
        //         HealthBar::MAX_HEIGHT,
        //     ),
        //     origin: RectangleOrigin::BottomLeft,
        // };

        // *path = ShapePath::build_as(&polygon);

        // *draw_mode = DrawMode::Outlined {
        //     fill_mode: FillMode::color({
        //         if healthbar.is_backdrop {
        //             Color::GRAY
        //         } else if (health.val as f32) <= (Health::MAX_HEALTH as f32) / 2.0 {
        //             Color::RED
        //         } else {
        //             Color::GREEN
        //         }
        //     }),
        //     outline_mode: StrokeMode::new(Color::BLACK, 1.0),
        // };
        gizmos.line_2d(
            Vec2::new(p_transform.translation.x - HealthBar::MAX_WIDTH / 2.0, p_transform.translation.y - Tank::RADIUS - 20.0),
            Vec2::new(p_transform.translation.x + HealthBar::MAX_WIDTH / 2.0, p_transform.translation.y - Tank::RADIUS - 20.0),
            Color::GRAY,
        );

        gizmos.line_2d(
            Vec2::new(p_transform.translation.x - HealthBar::MAX_WIDTH / 2.0, p_transform.translation.y - Tank::RADIUS - 20.0),
            Vec2::new(p_transform.translation.x - HealthBar::MAX_WIDTH / 2.0 + HealthBar::MAX_WIDTH * (health.val as f32) / (Health::MAX_HEALTH as f32), p_transform.translation.y - Tank::RADIUS - 20.0),
           
            if (health.val as f32) <= (Health::MAX_HEALTH as f32) / 2.0 {
                Color::RED
            } else {
                Color::GREEN
            },
        );

        // transform.translation.x = p_transform.translation.x - HealthBar::MAX_WIDTH / 2.0;
        // transform.translation.y = p_transform.translation.y - Tank::RADIUS - 10.0;
    }
}
