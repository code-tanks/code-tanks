use bevy::prelude::*;
use ctsimlib::{c_tank::AllTankInfo, create_gun, create_radar, c_client::Client, create_base_tank};

use crate::DockerClient;

// use crate::{
//     c_client::{Client, DockerClient},
//     c_tank::Gun,
//     c_tank::Radar,
//     c_tank::Tank,
//     c_tank::{AllTankInfo, DamageDealer, TankInfo},
//     CCollider, CollisionType, Game,
// };
// use bevy_rapier2d::prelude::*;

// use crate::{c_command_source::CommandSource, c_event::EventSink, c_health::Health, CollisionMask};


// pub fn create_basic_tank(id: String, i: usize, client: impl Component, commands: &mut Commands) {
//     let x = 150.0 * (i as f32) + 10.0;
//     let y = 0.0;

//     let gun = create_gun(commands, x, y);

//     let radar = create_radar(commands, x, y);

//     create_base_tank(id, i, commands, gun, radar, x, y, client);
// }

pub fn setup_sim_tanks(state: Res<AllTankInfo>, mut commands: Commands) {
    // let game_url = state.all.iter().map(|f| f.hash.to_string()).collect::<Vec<String>>().join("");

    for tank_info in state.all.iter() {
        // create_basic_tank(
        //     tank_id.to_string(),
        //     i,
        //     Client {
        //         client: Box::new(DockerClient {
        //             tank_container_name: tank_container_name.to_string(),
        //         }),
        //     },
        //     &mut commands,
        // );
        let x = 150.0 * (tank_info.index as f32) + 10.0;
        let y = 0.0;

        let gun = create_gun(&mut commands, x, y);
        let radar = create_radar(&mut commands, x, y);

        let client = Client {
            client: Box::new(DockerClient {
                tank_container_name: tank_info.container_name.to_string(),
            }),
        };
        create_base_tank(tank_info, &mut commands, gun, radar, x, y, client);
    }
}
