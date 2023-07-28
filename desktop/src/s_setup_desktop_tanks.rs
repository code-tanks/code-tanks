use std::iter::zip;

use bevy::prelude::{AssetServer, Commands, Res, ResMut, Assets, Mesh};
use bevy::sprite::ColorMaterial;
use ctsimlib::c_tank::{AllTankInfo, TankInfo};
use ctsimlib::{c_client::Client, *};
use ctsimlibgraphics::*;

use crate::{PORTS, UseDummy, DummyClient, DesktopClient};

pub fn setup_desktop_tanks(
    // mut state: ResMut<TickState>,
    state: Res<AllTankInfo>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,    
    // use_dummy: Res<UseDummy>  
) {
    create_environment(&mut commands, &asset_server);

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
        // println!("{} {}", tank_id, tank_container_name);
        create_graphics_tank(
            &mut commands,
            tank_info,
            Client {
                client: Box::new(DesktopClient {
                    info: tank_info.clone(),
                }),
            },
            &asset_server,
            // tank_id.to_string(),
            &mut meshes,
            &mut materials,
        );
    }
}

