pub mod c_client;
pub mod c_collider;
pub mod c_command;
pub mod c_event;
pub mod c_health;
pub mod c_position;
pub mod c_render;
pub mod c_scanner;
pub mod c_tank;
pub mod c_velocity;

pub mod s_apply_commands;
pub mod s_physics;
pub mod s_publish_events;
pub mod s_render;
pub mod s_request_commands;
pub mod s_save_commands;

use c_client::*;
use c_collider::*;
use c_command::*;
use c_event::*;
use c_health::*;
use c_position::*;
use c_render::*;
use c_scanner::*;

use c_velocity::*;

use std::fs::File;
use std::io::{prelude::*, BufReader};

use s_apply_commands::*;
use s_physics::*;
use s_publish_events::*;
use s_render::*;
use s_request_commands::*;
use s_save_commands::*;

use bevy_ecs::prelude::*;

pub struct Game {
    pub world: World,
    pub entities: Vec<Entity>,
}

pub fn create_game(urls: &[String]) -> Game {
    let mut world = World::default();
    let entities = urls
        .iter()
        .map(|_url| {
            world
                .spawn()
                .insert(Render::as_tank())
                .insert(Health {})
                .insert(Position {
                    x: 0.0,
                    y: 0.0,
                    rotation: 0.0,
                })
                .insert(CVelocity { velocity: 0.0 })
                .insert(TankVelocity {
                    angular_velocity: 0.0,
                    gun_angular_velocity: 0.0,
                    radar_angular_velocity: 0.0,
                })
                .insert(CCollider::tank())
                .insert(CommandSource::default())
                .insert(Client {
                    client: Box::new(DummyClient {}),
                })
                .insert(Scanner {})
                .insert(EventSink::default())
                // .insert(TankUtilities {})
                .id()
        })
        .collect();
    Game { world, entities }
}

pub fn run_game(game: &mut Game) -> &mut Game {
    let mut schedule = Schedule::default();

    let mut f = File::create("./sim.txt").expect("Unable to create file");
    f.write_all(format!("{}\n", game.entities.len()).to_string().as_bytes())
        .expect("Unable to write data");

    // Add a Stage to our schedule. Each Stage in a schedule runs all of its systems
    // before moving on to the next Stage
    schedule.add_stage("render", SystemStage::single_threaded().with_system(render));

    schedule.add_stage(
        "request_commands",
        SystemStage::single_threaded().with_system(request_commands),
    );
    schedule.add_stage(
        "save_commands",
        SystemStage::single_threaded().with_system(save_commands),
    );
    schedule.add_stage(
        "apply_commands",
        SystemStage::single_threaded().with_system(apply_commands),
    );

    schedule.add_stage(
        "physics",
        SystemStage::single_threaded().with_system(physics),
    );

    // schedule.add_stage(
    //     "scanner",
    //     SystemStage::single_threaded().with_system(scanner),
    // );

    schedule.add_stage(
        "publish_events",
        SystemStage::single_threaded().with_system(publish_events),
    );

    for _ in 0..2000 {
        // Run the schedule once. If your app has a "loop", you would run this once per loop
        schedule.run(&mut game.world);
    }

    game
}
pub fn run_view_game(file: &str) {
    let file = File::open(file).unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    // while let Some(line) = reader.read_line(&mut buffer) {
    //     println!("{}", line?.trim());
    // }

    let s: usize = lines[0].parse::<usize>().unwrap();
    println!("# players: {}", s);

    let mut n_commands = 0;

    let mut world = World::default();
    let entities = (0..s)
        .map(|n| {
            let c_lines: Vec<CCommand> = lines[(1 + n)..]
                .iter()
                .step_by(s)
                .map(|f| f.to_string().parse::<CCommand>().unwrap())
                .collect();
            println!("{} lines: {:?}", n + 1, c_lines);
            if n_commands == 0 && c_lines.len() > 0 {
                n_commands = c_lines.len();
            }
            assert!(n_commands == c_lines.len());

            world
                .spawn()
                .insert(Render::as_tank())
                .insert(Health {})
                .insert(Position {
                    x: 0.0,
                    y: 0.0,
                    rotation: 0.0,
                })
                .insert(CVelocity { velocity: 0.0 })
                .insert(TankVelocity {
                    angular_velocity: 0.0,
                    gun_angular_velocity: 0.0,
                    radar_angular_velocity: 0.0,
                })
                .insert(CCollider::tank())
                .insert(CommandSource::default())
                .insert(Client {
                    client: Box::new(ReaderClient { lines: c_lines }),
                })
                .insert(Scanner {})
                .insert(EventSink::default())
                // .insert(TankUtilities {})
                .id()
        })
        .collect();
    let mut game = Game { world, entities };

    let mut schedule = Schedule::default();

    // Add a Stage to our schedule. Each Stage in a schedule runs all of its systems
    // before moving on to the next Stage
    schedule.add_stage("render", SystemStage::single_threaded().with_system(render));

    schedule.add_stage(
        "request_commands",
        SystemStage::single_threaded().with_system(request_commands),
    );

    schedule.add_stage(
        "apply_commands",
        SystemStage::single_threaded().with_system(apply_commands),
    );

    schedule.add_stage(
        "physics",
        SystemStage::single_threaded().with_system(physics),
    );

    // schedule.add_stage(
    //     "scanner",
    //     SystemStage::single_threaded().with_system(scanner),
    // );

    schedule.add_stage(
        "publish_events",
        SystemStage::single_threaded().with_system(publish_events),
    );

    for _ in 0..n_commands {
        // Run the schedule once. If your app has a "loop", you would run this once per loop
        schedule.run(&mut game.world);
    }
}
