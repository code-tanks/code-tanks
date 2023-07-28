use std::{process::Command as ProcessCommand, thread, time};

use bevy::{
    prelude::{default, App, Resource, Startup},
    winit::WinitSettings,
};
use bevy_rapier2d::prelude::RapierDebugRenderPlugin;
use ct_api::Command;
use ct_api::Commands;
use ctsimlib::{
    c_client::{parse_commands, ClientTrait},
    c_event::CTEvent,
    c_tank::{AllTankInfo, TankInfo},
    s_setup_walls::setup_walls,
    *,
};
use s_setup_desktop_tanks::setup_desktop_tanks;

pub mod s_setup_desktop_tanks;
use ctsimlib::core_plugin::CoreCTPlugin;
use ctsimlibgraphics::CoreCTGraphicsPlugin;

const PORTS: [usize; 4] = [8061, 8062, 8063, 8064];

#[derive(Default, Resource)]
pub struct UseDummy {
    pub use_dummy: bool,
}

pub fn run_game(tank_hashes: &[String]) {
    let game_url: String = tank_hashes.join("");
    let tank_infos = &tank_hashes
        .iter()
        .enumerate()
        .map(|(i, f)| TankInfo {
            hash: f.to_string(),
            id: format!("{}-{}", f, i),
            index: i,
            container_name: format!("{}-{}-{}", game_url, f, i),
        })
        .collect::<Vec<TankInfo>>();

    // let tank_container_names = tank_hashes
    //     .iter()
    //     .enumerate()
    //     .map(|(i, url)| run_local_tank(url, &game_url, i, PORTS[i]))
    //     .collect::<Vec<String>>();

    for tank_info in tank_infos {
        // TODO fix
        let tank_image_name = &tank_info.hash;
        run_tank(
            &tank_info.container_name,
            tank_image_name,
            &format!("{}:8080", PORTS[tank_info.index]),
            false
        );
    }
    thread::sleep(time::Duration::from_millis(1000));

    App::new()
        .insert_resource(WinitSettings {
            return_from_run: true,
            ..default()
        })
        .add_plugins(CoreCTPlugin)
        .add_plugins(CoreCTGraphicsPlugin)
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup_desktop_tanks, setup_walls))
        // .insert_resource(UseDummy {
        //     use_dummy: tank_hashes.is_empty(),
        // })
        .insert_resource(AllTankInfo {
            all: tank_infos.to_vec(),
        })
        .run();

    for tank_info in tank_infos {
        remove_tank(&tank_info.container_name);
        println!("removed {}", &tank_info.container_name);
    }

    println!("finished");
}

pub struct DummyClient {}

impl ClientTrait for DummyClient {
    fn request_commands(&mut self) -> Vec<Command> {
        vec![Commands::NONE]
    }

    fn request_commands_by_event(&mut self, _event: &CTEvent) -> Vec<Command> {
        self.request_commands()
    }
}

pub struct DesktopClient {
    pub info: TankInfo,
}

impl ClientTrait for DesktopClient {
    fn request_commands(&mut self) -> Vec<Command> {
        let output = ProcessCommand::new("bash")
            .arg("-c")
            .arg(format!(
                r#"curl localhost:{}/request_commands | jq --raw-output '.[]'"#,
                PORTS[self.info.index],
            ))
            .output()
            .expect("failed to communicate with tank");

        if output.status.success() {
            let result_raw = String::from_utf8_lossy(&output.stdout);
            return parse_commands(result_raw.to_string());
        }

        let _err_raw = String::from_utf8_lossy(&output.stderr);
        println!(
            "SELF_DESTRUCT {:?} empty request_commands",
            self.info.container_name
        );
        vec![Commands::SELF_DESTRUCT]
    }

    fn request_commands_by_event(&mut self, event: &CTEvent) -> Vec<Command> {
        let output = ProcessCommand::new("bash")
            .arg("-c")
            .arg(format!(
                r#"curl -d '{}' -X POST localhost:{}/request_commands_by_event | jq --raw-output '.[]'"#,
                serde_json::to_string(event).unwrap(),
                PORTS[self.info.index],
            ))
            .output()
            .expect("failed to communicate with ocypod");

        if output.status.success() {
            let result_raw = String::from_utf8_lossy(&output.stdout);
            return parse_commands(result_raw.to_string());
        }
        let _err_raw = String::from_utf8_lossy(&output.stderr);
        vec![]
    }
}
