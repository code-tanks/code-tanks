use std::iter::zip;

use bevy::prelude::{AssetServer, Commands, Res, ResMut, Assets, Mesh};
use bevy::sprite::ColorMaterial;
use ctsimlib::c_client::{DesktopClient, DummyClient};
use ctsimlib::{c_client::Client, *};
use ctsimlibgraphics::*;

use crate::{PORTS, UseDummy};

pub fn setup_desktop_tanks(
    // mut state: ResMut<TickState>,
    tank_state: Res<TankInfo>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,    
    use_dummy: Res<UseDummy>  
) {
    create_environment(&mut commands, &asset_server);

    if use_dummy.use_dummy {
        create_graphics_tank(
            &mut commands,
            0,
            Client {
                client: Box::new(DummyClient {}),
            },
            &asset_server,
            "dummy".to_string(),
            &mut meshes,
            &mut materials,
        );
        return;
    }

    for (i, (tank_id, tank_nametag)) in zip(
        tank_state.tank_ids.clone(),
        tank_state.tank_nametags.clone(),
    )
    .enumerate()
    {
        println!("{} {}", tank_id, tank_nametag);
        create_graphics_tank(
            &mut commands,
            i,
            Client {
                client: Box::new(DesktopClient {
                    tank_nametag: tank_nametag.to_string(),
                    port: PORTS[i],
                }),
            },
            &asset_server,
            tank_id.to_string(),
            &mut meshes,
            &mut materials,
        );
    }
}
