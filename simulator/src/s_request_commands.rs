use bevy::prelude::Query;

use crate::{c_client::Client, c_command::CommandSource, c_health::Health};

pub fn request_commands(mut query: Query<(&mut CommandSource, &mut Client, &mut Health)>) {
    for (mut command_receiver, mut client_connection, mut health) in &mut query {
        if health.val == 0 {
            continue;
        }

        if command_receiver.queue.is_empty() {
            let mut new_commands = client_connection.client.request_commands();
            if new_commands.is_empty() {
                health.val = 0;
            } else {
                command_receiver.queue.append(&mut new_commands);
            }
        }
        // println!("request_commands {:?}", command_receiver.queue);
    }
}
