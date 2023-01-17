use bevy::prelude::{Query, Entity};

use crate::{c_client::Client, c_command::{CommandSource, CCommands}, c_health::Health};

pub fn request_commands(mut query: Query<(Entity, &mut CommandSource, &mut Client, &Health)>) {
    for (entity, mut command_receiver, mut client_connection, health) in &mut query {
        if command_receiver.queue.is_empty() {
            if health.val == 0 {
                command_receiver.queue.push(CCommands::NONE);
                continue;
            }

            let mut new_commands = client_connection.client.request_commands();
            // println!("request_commands {:?} {:?}", entity, new_commands);
            if new_commands.is_empty() {
                println!("SELF_DESTRUCT {:?} empty request_commands", entity);
                command_receiver.queue.push(CCommands::SELF_DESTRUCT);
            } else {
                command_receiver.queue.append(&mut new_commands);
            }
        }
        // println!("commands {:?} {:?}", entity, command_receiver.queue);
    }
}
