use bevy::prelude::Query;

use crate::{c_client::Client, c_command::CommandSource};

pub fn request_commands(mut query: Query<(&mut CommandSource, &mut Client)>) {
    for (mut command_receiver, mut client_connection) in &mut query {
        if command_receiver.queue.is_empty() {
            command_receiver
                .queue
                .append(&mut client_connection.client.request_commands());
        }
        // println!("request_commands {:?}", command_receiver.queue);
    }
}
