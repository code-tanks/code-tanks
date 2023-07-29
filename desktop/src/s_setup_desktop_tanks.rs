
use std::{thread, time};

use bevy::prelude::{AssetServer, Commands, Res, ResMut, Assets, Mesh};
use bevy::sprite::ColorMaterial;
use ctsimlib::c_tank::{AllTankInfo, TankInfo};
use ctsimlib::c_client::Client;
use ctsimlib::run_tank;
use ctgraphics::*;

use crate::{DummyClient, DesktopClient, get_free_port};

pub fn setup_desktop_tanks(
    // mut state: ResMut<TickState>,
    state: Res<AllTankInfo>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,    
    // use_dummy: Res<UseDummy>  
) {
    // create_environment(&mut commands, &asset_server);

    if state.all.is_empty() {
        create_graphics_tank(
            &mut commands,
            &TankInfo {
                hash: "dummy".to_string(),
                id: "dummy-0".to_string(),
                index: 0,
                container_name: "dummy-dummy-0".to_string(),
            },
            Client {
                client: Box::new(DummyClient {}),
            },
            &asset_server,
            &mut meshes,
            &mut materials,
        );
        return;
    }

    for tank_info in state.all.iter() {
        let tank_image_name = &tank_info.hash;
        let port = get_free_port();
        println!("got free port: {}", port);
        run_tank(
            &tank_info.container_name,
            tank_image_name,
            &format!("{}:8080", port),
            false
        );        
        create_graphics_tank(
            &mut commands,
            tank_info,
            Client {
                client: Box::new(DesktopClient {
                    info: tank_info.clone(),
                    port: port,
                }),
            },
            &asset_server,
            &mut meshes,
            &mut materials,
        );
    }

    thread::sleep(time::Duration::from_millis(1000));

}

