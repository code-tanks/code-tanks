use bevy::prelude::*;

use crate::*;

use ctsimlib::c_tank::*;

pub fn apply_history_transforms(
    // mut commands: Commands,
    mut query: Query<
    (
        &mut Transform,
        &Tank,
        &mut HistoryTransforms
    ),
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
        t.rotation.x = transforms[2];
        t.rotation.y = transforms[3];
        t.rotation.z = transforms[4];
        t.rotation.w = transforms[5];

        let mut radar = query_t.get_mut(tank.radar).unwrap();
        radar.rotation.x = transforms[6];
        radar.rotation.y = transforms[7];
        radar.rotation.z = transforms[8];
        radar.rotation.w = transforms[9];

        let mut gun = query_t.get_mut(tank.gun).unwrap();
        gun.rotation.x = transforms[10];
        gun.rotation.y = transforms[11];
        gun.rotation.z = transforms[12];
        gun.rotation.w = transforms[13];
    }
}