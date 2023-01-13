use bevy::prelude::*;
use ctsimlib::c_tank::Tank;

use crate::c_nametag::NameTag;

pub fn update_nametag(
    q_parent: Query<&Transform, With<Tank>>,
    mut q: Query<(&mut Transform, &NameTag), Without<Tank>>,
) {
    for (mut transform, nametag) in &mut q {
        let p_transform = q_parent.get(nametag.tank).unwrap();

        transform.translation.x = p_transform.translation.x;
        transform.translation.y = p_transform.translation.y - Tank::HEIGHT;
    }
}
