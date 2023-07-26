use std::env;
// use bevy::math::Vec3;
// use bevy::math::f32::Quat;

use ctdesktop::run_game;

fn main() {
    let args: Vec<String> = env::args().collect();

    let args = &args[1..];
    println!("running game: {:?}", args);
    run_game(args);

    // let _v = Quat::from_xyzw(0.,0.,-0.70710677,0.70710677) * Vec3::Y;
    // println!("angle: {}", _v.y.atan2(_v.x));
    // let _v = Quat::from_xyzw(0.,0.,-0.71263844,0.70153147) * Vec3::Y;
    // println!("angle: {}", _v.y.atan2(_v.x));
}
