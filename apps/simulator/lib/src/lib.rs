pub mod client_connection;
pub mod collider;
pub mod command_receiver;
pub mod event_sender;
pub mod health;
pub mod position;
pub mod render;
pub mod scanner;
pub mod tank_utilities;
pub mod velocity;

use std::f32::consts::TAU;

use client_connection::*;
use collider::*;
use command_receiver::*;
use event_sender::*;
use health::*;
use position::*;
use render::*;
use scanner::*;
use tank_utilities::*;
use velocity::*;

use bevy_ecs::prelude::*;

pub struct Game {
    pub world: World,
    pub entities: Vec<Entity>,
}

pub fn create_game() -> Game {
    let mut world = World::default();
    let entities = (1..4u8)
        .map(|_| {
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
                .insert(CommandReceiver::default())
                .insert(ClientConnection::dummy())
                .insert(Scanner {})
                .insert(EventSender::default())
                // .insert(TankUtilities {})
                .id()
        })
        .collect();
    Game { world, entities }
}

fn render(query: Query<(Entity, &Render, &Position)>) {
    for (entity, render, position) in &query {
        println!(
            "render {:?}, {:?}, {:?}",
            entity.id(),
            render.render_type,
            position
        );
    }
}

fn request_commands(mut query: Query<(&mut CommandReceiver, &ClientConnection)>) {
    for (mut command_receiver, client_connection) in &mut query {
        if command_receiver.queue.is_empty() {
            command_receiver
                .queue
                .append(&mut client_connection.client.request_commands());
        }
        println!("request_commands {:?}", command_receiver.queue);
    }
}

fn apply_commands(
    mut query: Query<(
        &mut CommandReceiver,
        &mut Velocity,
        &mut TankVelocity,
        &Position,
    )>,
) {
    for (mut command_receiver, mut velocity, mut tank_velocity, position) in &mut query {
        let grouped_commands = &mut command_receiver.queue[0];

        println!("apply_commands {:?}", grouped_commands);

        for command_type_as_usize in 0..COMMAND_TYPES_LENGTH {
            if grouped_commands.command_array[command_type_as_usize] > 0 {
                grouped_commands.command_array[command_type_as_usize] =
                    grouped_commands.command_array[command_type_as_usize] - 1;

                let command_type: CommandType =
                    unsafe { ::std::mem::transmute(command_type_as_usize) };

                match command_type {
                    CommandType::None => {}
                    CommandType::MoveForward => {
                        println!("AheadBy");

                        velocity.velocity = 1.0;
                    }
                    CommandType::MoveBackward => {
                        velocity.velocity = -1.0;
                    }
                    CommandType::RotateTankClockwise => {
                        tank_velocity.angular_velocity = 1.0;
                    }
                    CommandType::RotateTankCounterClockwise => {
                        tank_velocity.angular_velocity = -1.0;
                    }
                    CommandType::RotateGunClockwise => {
                        tank_velocity.angular_velocity = 1.0;
                    }
                    CommandType::RotateGunCounterClockwise => {
                        tank_velocity.angular_velocity = 1.0;
                    }
                    CommandType::RotateRaderClockwise => {
                        tank_velocity.angular_velocity = 1.0;
                    }
                    CommandType::RotateRaderCounterClockwise => {
                        tank_velocity.angular_velocity = 1.0;
                    }
                    CommandType::FireWithPower => {}
                }
            }
        }

        if !grouped_commands.command_array.iter().any(|x| x > &0) {
            command_receiver.queue.remove(0);
        }
        println!("commands remaining {:?}", command_receiver.queue);
    }
}

fn physics(mut query: Query<(&mut Velocity, &mut Position, &Collider, &mut TankVelocity)>) {
    for (mut velocity, mut position, collider, mut tank_velocity) in &mut query {
        // physComp
        // ..position.features[0] += physComp.velocity * -sin(physComp.rotation)
        // ..position.features[1] += physComp.velocity * cos(physComp.rotation)
        // ..rotation = (physComp.rotation + rotationDelta) % tau
        // ..velocity *= 0
        // ..angularVelocity *= 0;
        position.x += velocity.velocity * -tank_velocity.angular_velocity.sin();
        position.y += velocity.velocity * tank_velocity.angular_velocity.cos();
        position.rotation = (position.rotation + tank_velocity.angular_velocity) % TAU;
        velocity.velocity = 0.0;
        tank_velocity.angular_velocity = 0.0;
    }
}

fn scanner(query: Query<(Entity, &Scanner, &Position, &Collider)>) {
    for (entity, scanner, position, collider) in &query {}
}

fn publish_events(mut query: Query<(&mut CommandReceiver, &EventSender, &ClientConnection)>) {
    for (mut command_receiver, event_sender, client_connection) in &mut query {
        let mut queue: Vec<GroupedCommand> = Vec::new();
        for event in event_sender.queue.iter() {
            queue.append(&mut client_connection.client.request_commands_by_event(event));
        }
        command_receiver.queue.splice(0..0, queue);
    }
}

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

    schedule.add_stage(
        "scanner",
        SystemStage::single_threaded().with_system(scanner),
    );

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
