use bevy::prelude::*;
use wasm_bindgen::prelude::*;

use crate::CState;

#[wasm_bindgen]
pub fn get_sim_file() -> String {
    web_sys::window()
        .unwrap()
        .location()
        .pathname()
        .unwrap()
        .split("/")
        .filter(|f| !f.is_empty())
        .map(|f| f.to_string())
        .collect::<Vec<String>>()
        .join("-")
}

pub fn load_tanks(mut state: ResMut<CState>, asset_server: Res<AssetServer>) {
    // state.handle = asset_server.load("./sim.txt");
    let file = format!("./sim/{}.txt", get_sim_file());

    println!("sim file: {}", file);
    info!("sim file: {}", file);

    state.handle = asset_server.load(&file);
}
