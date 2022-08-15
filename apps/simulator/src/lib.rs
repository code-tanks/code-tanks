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

use c_client::*;
use c_collider::*;
use c_command::*;
use c_event::*;
use c_health::*;
use c_position::*;
use c_render::*;
use c_scanner::*;
use c_tank::*;
use c_velocity::*;
use std::io::{self, prelude::*, BufReader};
use std::{f32::consts::TAU, fs::File};

use s_apply_commands::*;
use s_physics::*;
use s_publish_events::*;
use s_render::*;
use s_request_commands::*;

use bevy_ecs::prelude::*;

pub struct Game {
    pub world: World,
    pub entities: Vec<Entity>,
}

pub fn create_game(urls: &[String]) -> Game {
    let mut world = World::default();
    let entities = urls
        .iter()
        .map(|url| {
            world
                .spawn()
                .insert(Render::as_tank())
                .insert(Health {})
                .insert(Position {
                    x: 0.0,
                    y: 0.0,
                    rotation: 0.0,
                })
                .insert(Velocity { velocity: 0.0 })
                .insert(TankVelocity {
                    angular_velocity: 0.0,
                    gun_angular_velocity: 0.0,
                    radar_angular_velocity: 0.0,
                })
                .insert(Collider::tank())
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

pub fn create_view_game(file: &str) -> Game {
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

    let mut world = World::default();
    let entities = (0..s)
        .map(|n| {
            let c_lines = lines[(1 + n)..]
                .iter()
                .step_by(s)
                .map(|f| f.to_string())
                .collect();
            println!("{} lines: {:?}", n + 1, c_lines);
            world
                .spawn()
                .insert(Render::as_tank())
                .insert(Health {})
                .insert(Position {
                    x: 0.0,
                    y: 0.0,
                    rotation: 0.0,
                })
                .insert(Velocity { velocity: 0.0 })
                .insert(TankVelocity {
                    angular_velocity: 0.0,
                    gun_angular_velocity: 0.0,
                    radar_angular_velocity: 0.0,
                })
                .insert(Collider::tank())
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
    Game { world, entities }
}

// fn scanner(query: Query<(Entity, &Scanner, &Position, &Collider)>) {
//     for (entity, scanner, position, collider) in &query {}
// }

pub fn run_game(game: &mut Game) -> &mut Game {
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

    for _ in 0..200u8 {
        // Run the schedule once. If your app has a "loop", you would run this once per loop
        schedule.run(&mut game.world);
    }

    game
}

mod my_reader {
    use std::{
        fs::File,
        io::{self, prelude::*},
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);

            Ok(Self { reader })
        }

        pub fn read_line<'buf>(
            &mut self,
            buffer: &'buf mut String,
        ) -> Option<io::Result<&'buf mut String>> {
            buffer.clear();

            self.reader
                .read_line(buffer)
                .map(|u| if u == 0 { None } else { Some(buffer) })
                .transpose()
        }
    }
}
