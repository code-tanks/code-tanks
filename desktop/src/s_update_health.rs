use bevy::prelude::{Children, Query, Vec2, With};
use bevy_prototype_lyon::{
    prelude::{Path, ShapePath},
    shapes::{self, RectangleOrigin},
};
use ctsimlib::{c_health::Health, c_healthbar::HealthBar};

pub fn update_health(
    q_parent: Query<(&Health, &Children)>,
    mut q_child: Query<&mut Path, With<HealthBar>>,
) {
    for (health, children) in q_parent.iter() {
        // `children` is a collection of Entity IDs
        for &child in children.iter() {
            // get the health of each child unit
            let p = q_child.get_mut(child);

            if p.is_ok() {
                let mut path = p.unwrap();
                let polygon = shapes::Rectangle {
                    extents: Vec2::new(
                        38.0 * (health.val as f32) / (Health::MAX_HEALTH as f32),
                        3.0,
                    ),
                    origin: RectangleOrigin::BottomLeft,
                };

                *path = ShapePath::build_as(&polygon);
            }
            // do something
        }
    }
}
