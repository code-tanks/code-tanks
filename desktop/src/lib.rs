use std::{process::Command as ProcessCommand, fs::File, io::Write};

use bevy::{
    prelude::*,
    winit::WinitSettings,
};
use bevy_rapier2d::prelude::RapierDebugRenderPlugin;
use ct_api::Command;
use ct_api::Commands;
use ctsimlib::{
    c_client::{parse_commands, ClientTrait},
    c_event::CTEvent,
    c_tank::{AllTankInfo, TankInfo},
    s_apply_commands::apply_commands,
    s_request_commands::request_commands,
    s_save_commands::save_commands,
    s_setup_walls::setup_walls,
    *, s_apply_history_transforms::apply_history_transforms,
};
use s_setup_desktop_tanks::setup_desktop_tanks;

pub mod s_setup_desktop_tanks;
use ctgraphics::{
    s_setup_graphics::setup_graphics, s_setup_ground::setup_ground, CoreCTGraphicsPlugin, s_setup_reader_tanks::setup_reader_tanks,
};
use ctsimlib::core_plugin::CoreCTPlugin;

// const PORTS: [usize; 4] = [8062, 8063, 8064, 8065];

#[derive(Default, Resource)]
pub struct UseDummy {
    pub use_dummy: bool,
}

pub fn run_game(tank_hashes: &[String]) {
    let game_url: String = tank_hashes.join("-");
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

    App::new()
        .insert_resource(WinitSettings {
            return_from_run: true,
            ..default()
        })
        .add_plugins(CoreCTPlugin)
        .add_plugins(CoreCTGraphicsPlugin)
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(
            Startup,
            (
                (setup_desktop_tanks, setup_walls, setup_ground),
                setup_graphics,
            )
                .chain(),
        )
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

pub fn run_game_and_save(tank_hashes: &[String], ticks: u32) {
    let game_url: String = tank_hashes.join("-");
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

    let mut f = File::create("./sim.txt").expect("Unable to create file");
    f.write_all(format!("{}\n", tank_hashes.join(",")).as_bytes())
        .expect("Unable to write data");

    App::new()
        .insert_resource(WinitSettings {
            return_from_run: true,
            ..default()
        })
        // .insert_resource(FixedTime::new_from_secs(TickState::DT))
        .insert_resource(MaxSimulationTicks(ticks))
        .add_plugins(CoreCTPlugin)
        .add_plugins(CoreCTGraphicsPlugin)
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(
            Startup,
            (
                (setup_desktop_tanks, setup_walls, setup_ground),
                setup_graphics,
            )
                .chain(),
        )
        .add_systems(
            Update,
            save_commands.after(request_commands).before(apply_commands),
        )
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

#[derive(Resource)]
pub struct SimFilePath(pub String);

pub fn load_tanks_from_file(mut state: ResMut<CustomAssetState>, asset_server: Res<AssetServer>, sim_file_path: Res<SimFilePath>) {
    // state.handle = asset_server.load("./sim.txt");
    let file = &sim_file_path.0;

    println!("sim file: {}", file);
    info!("sim file: {}", file);

    state.handle = asset_server.load(file);

    info!("got file");
}



pub fn read_game(file: &str) {


    App::new()
        .insert_resource(WinitSettings {
            return_from_run: true,
            ..default()
        })
        // .insert_resource(FixedTime::new_from_secs(TickState::DT))
        .add_plugins(CoreCTPlugin)
        .add_plugins(CoreCTGraphicsPlugin)
        // .add_plugins(RapierDebugRenderPlugin::default())
        .init_resource::<CustomAssetState>()
        .insert_resource(SimFilePath(file.to_string()))
        .add_asset::<CustomAsset>()
        .init_asset_loader::<CustomAssetLoader>()
        .add_systems(Startup, (load_tanks_from_file, setup_walls, setup_ground))
        .add_systems(
            Update,
            (setup_reader_tanks.before(apply_commands), apply_history_transforms.after(request_commands).before(apply_commands))
            // "request_commands",
            // "apply_history_transforms",
            // SystemStage::single_threaded().with_system(apply_history_transforms),
        )
        .run();

    // for tank_info in tank_infos {
    //     remove_tank(&tank_info.container_name);
    //     println!("removed {}", &tank_info.container_name);
    // }

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

fn get_free_port() -> String {
    let output = ProcessCommand::new("bash")
        .arg("-c")
        .arg(r#"comm -23 <(seq 8000 9000 | sort) <(ss -Htan | awk '{print $4}' | cut -d':' -f2 | sort -u) | shuf | head -n 1"#,
        )
        .output()
        .expect("failed to communicate with tank");

    let result_raw = String::from_utf8_lossy(&output.stdout);
    result_raw.trim().to_string()
}

pub struct DesktopClient {
    pub info: TankInfo,
    pub port: String,
}

impl ClientTrait for DesktopClient {
    fn request_commands(&mut self) -> Vec<Command> {
        let output = ProcessCommand::new("bash")
            .arg("-c")
            .arg(format!(
                r#"curl -sS -m 3 localhost:{}/request_commands | jq --raw-output '.[]'"#,
                self.port,
            ))
            .output()
            .expect("failed to communicate with tank");

        let err_raw = String::from_utf8_lossy(&output.stderr);

        if err_raw.is_empty() {
            let result_raw = String::from_utf8_lossy(&output.stdout);
            let commands = parse_commands(result_raw.to_string());
            if commands.is_empty() {
                return vec![Commands::SELF_DESTRUCT];
            } else {
                return commands;
            }
        }

        // let _err_raw = String::from_utf8_lossy(&output.stderr);
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
                r#"curl -sS -m 3 -d '{}' -X POST localhost:{}/request_commands_by_event | jq --raw-output '.[]'"#,
                serde_json::to_string(event).unwrap(),
                self.port,
            ))
            .output()
            .expect("failed to communicate with ocypod");
        let err_raw = String::from_utf8_lossy(&output.stderr);

        if err_raw.is_empty() {
            let result_raw = String::from_utf8_lossy(&output.stdout);
            return parse_commands(result_raw.to_string());
        }
        // let _err_raw = String::from_utf8_lossy(&output.stderr);
        vec![]
    }
}
