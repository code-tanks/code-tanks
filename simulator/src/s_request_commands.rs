use bevy::prelude::Query;
use ct_api::Commands;

use crate::{c_client::Client, c_command::CommandSource, c_health::Health};

pub fn request_commands(mut query: Query<(&mut CommandSource, &mut Client, &Health)>) {
    // println!("request commands");
    for (mut command_receiver, mut client_connection, health) in &mut query {
        if command_receiver.queue.is_empty() {
            if health.val == 0 {
                command_receiver.queue.push(Commands::NONE);
                continue;
            }

            let mut new_commands = client_connection.client.request_commands();
            command_receiver.queue.append(&mut new_commands);
        }
        // println!("commands {:?}", command_receiver.queue);
    }
}
