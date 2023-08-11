use bevy::prelude::*;
use ctsimlib::{c_tank::{Tank, Radar}, c_health::Health, c_radar_needs_update::RadarNeedsUpdate};

use crate::s_on_added_bullet::{DISABLED_COLOR, COLORS};
// use bevy_prototype_lyon::prelude::{DrawMode, FillMode};

// use crate::c_particle::Particle;




pub fn update_radar(
    mut commands: Commands,
    query_tank: Query<(&Tank, &Health), Without<Radar>>,
    mut query_color: Query<&mut Handle<ColorMaterial>>,
    mut query_radar_with_update: Query<(Entity, &mut Radar), With<RadarNeedsUpdate>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

) {
    // for radar in &query_radar  {
    //     let health = query_tank.get();
    // }
    for (tank, health) in &query_tank {
        if let Ok((radar_entity, mut radar)) = query_radar_with_update.get_mut(tank.radar) {

            // if (hea)
            if let Ok(handle) = query_color.get_mut(radar_entity) {
                let mat = materials.get_mut(&handle).unwrap();
                // your color changing logic here instead:

                if health.val == 0 {
                    // color.set_a(color.a() * 0.99);
                    mat.color = DISABLED_COLOR.with_a(0.0);
                    radar.disabled = true;
                } else {
                    if radar.disabled {
                        mat.color = DISABLED_COLOR.with_a(0.3);
                    } else {
                        mat.color = COLORS[tank.info.index % COLORS.len()].with_a(0.3);
                    }
                }

                // color.set_a(color.a() * 0.99);
                // etc
            }

            // remove needsupdate component
            commands.entity(radar_entity).remove::<RadarNeedsUpdate>();
        }
    }
    // for (entity, mut sprite, mut particle) in &mut query {
    //     if particle.progress > particle.max_life_in_ticks {
    //         commands.entity(entity).despawn_recursive();
    //     }

    //     let opacity = Track::OPACITY
    //         * (1.0 - (particle.progress as f32 / particle.max_life_in_ticks as f32).powf(3.));

    //     // * = DrawMode::Fill(FillMode::color(Color::rgba(0., 0., 0., opacity)));
    //     sprite.color = Color::rgba(0., 0., 0., opacity);

    //     particle.progress += 1;
    // }
    // shape::Quad::new(Vec2::new(50., 100.));
}
