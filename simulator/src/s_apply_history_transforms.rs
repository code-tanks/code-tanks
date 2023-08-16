use bevy::prelude::*;


#[derive(Component)]
pub struct HistoryTransforms {
    pub transforms: Vec<Vec<f32>>,
}

use crate::c_tank::*;

pub fn apply_history_transforms(
    // mut commands: Commands,
    mut query: Query<
        (&mut Transform, &Tank, &mut HistoryTransforms),
        // (Without<Radar>, Without<Gun>),
    >,
    mut query_t: Query<&mut Transform, Without<Tank>>,
) {
    for (mut t, tank, mut history_transforms) in &mut query {
        if history_transforms.transforms.is_empty() {
            break;
        }

        let transforms = history_transforms.transforms.pop().unwrap();

        t.translation.x = transforms[0];
        t.translation.y = transforms[1];
        // t.rotation.x = transforms[2]; // unused
        // t.rotation.y = transforms[3]; // unused
        t.rotation.z = transforms[2];
        t.rotation.w = transforms[3];

        let mut radar = query_t.get_mut(tank.radar).unwrap();
        // radar.rotation.x = transforms[6]; // unused
        // radar.rotation.y = transforms[7]; // unused
        radar.rotation.z = transforms[4];
        radar.rotation.w = transforms[5];

        let mut gun = query_t.get_mut(tank.gun).unwrap();
        // gun.rotation.x = transforms[10]; // unused
        // gun.rotation.y = transforms[11]; // unused
        gun.rotation.z = transforms[6];
        gun.rotation.w = transforms[7];
    }
}
