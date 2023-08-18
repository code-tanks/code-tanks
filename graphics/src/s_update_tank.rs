use bevy::prelude::*;
use ctengine::{c_health::Health, c_tank::{Tank, Gun}};

pub fn update_tank(
    // mut commands: Commands,
    mut query_tank: Query<(&Children, &Tank, &Health), Without<Gun>>,
    mut query_sprite: Query<&mut Sprite, Without<Gun>>,
    mut query_gun: Query<&mut Sprite, With<Gun>>,

    // mut query: Query<(&mut Transform, &mut Tracks), With<Tank>>,
) {
    '_outer: for (children, tank, health) in &mut query_tank {
        'inner: for &child in children {
            if let Ok(mut tank_sprite) = query_sprite.get_mut(child) {
                if health.val == 0 {
                    let gun_sprite: &mut Mut<'_, Sprite> = &mut query_gun.get_mut(tank.gun).unwrap();
                    tank_sprite.color = Color::BLACK.with_a(0.75);
                    gun_sprite.color = Color::BLACK.with_a(0.75);
                }
                break 'inner;
            }
        }

        
    }
}