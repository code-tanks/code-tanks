use bevy::prelude::{Query, Entity};

use crate::{c_client::Client, c_command::CommandSource, c_health::Health};

pub fn request_commands(mut query: Query<(Entity, &mut CommandSource, &mut Client, &mut Health)>) {
    for (entity, mut command_receiver, mut client_connection, mut health) in &mut query {
        if health.val == 0 {
            println!("DEAD");
            continue;
        }

        if command_receiver.queue.is_empty() {
            let mut new_commands = client_connection.client.request_commands();
            println!("request_commands {:?} {:?}", entity, new_commands);
            if new_commands.is_empty() {
                println!("killed {:?} empty request_commands", entity);
                // TODO what do we do when request_commands returns empty?
                health.val = 0;
            } else {
                command_receiver.queue.append(&mut new_commands);
            }
        }
        println!("commands {:?} {:?}", entity, command_receiver.queue);
    }
}
