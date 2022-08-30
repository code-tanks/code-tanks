use bevy::prelude::{
    AssetServer, Commands, Res, ResMut,
};
use ctsimlib::{c_client::Client, *};
use ctsimlib::c_client::LocalClient;
use ctsimlibgraphics::*;

pub fn setup_desktop_tanks(
    mut state: ResMut<TickState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    state.tick = state.tick + 1;
    if state.tick > 1 {
        return;
    }

    for (i, tank_id) in state.tank_ids.iter().enumerate() {
        // create_tank(
        //     &mut commands,
        //     &asset_server,
        //     Client {
        //         client: Box::new(LocalClient {
        //             tank_id: tank_id.to_string(),
        //             port: i,
        //         }),
        //     },
        //     150.0 * (i as f32) + 10.0,
        //     0.0,
        // );
        create_graphics_tank(
            &mut commands,
            i,
            Client {
                client: Box::new(LocalClient {
                    tank_id: tank_id.to_string(),
                    port: i,
                }),
            },
            &asset_server,
        );
    }
}
