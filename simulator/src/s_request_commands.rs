use bevy::prelude::Query;

use crate::{c_client::Client, c_command::CommandSource, c_health::Health};

pub fn request_commands(mut query: Query<(&mut CommandSource, &mut Client, &Health)>) {
    for (mut command_receiver, mut client_connection, health) in &mut query {
        if health.val == 0 {
            continue;
        }

        if command_receiver.queue.is_empty() {
            command_receiver
                .queue
                .append(&mut client_connection.client.request_commands());
        }
        // println!("request_commands {:?}", command_receiver.queue);
    }
}
