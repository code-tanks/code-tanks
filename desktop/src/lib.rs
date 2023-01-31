use std::{thread, time};

use bevy::{prelude::*, winit::WinitSettings};
use ctsimlib::{s_setup_walls::setup_walls, *};
use s_setup_desktop_tanks::setup_desktop_tanks;

pub mod s_setup_desktop_tanks;
use ctsimlib::core_plugin::CoreCTPlugin;
use ctsimlibgraphics::CoreCTGraphicsPlugin;

pub fn run_game(args: &[String]) {
    let game_url = args.join("");
    let tank_ids = args
        .iter()
        .enumerate()
        .map(|(i, url)| run_tank(url, &game_url, i, true))
        .collect::<Vec<String>>();

    thread::sleep(time::Duration::from_millis(1000));

    App::new()
        .insert_resource(WinitSettings {
            return_from_run: true,
            ..default()
        })
        .add_plugin(CoreCTPlugin)
        .add_plugin(CoreCTGraphicsPlugin)
        .insert_resource(TankIds {
            tank_ids: tank_ids.to_vec(),
        })
        .add_system(setup_desktop_tanks)
        .add_startup_system(setup_walls)
        .run();

    for tank_id in tank_ids {
        remove_tank(&tank_id);
        println!("removed {}", &tank_id);
    }

    println!("finished");
}
