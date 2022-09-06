use bevy::prelude::*;

use crate::{
    c_client::Client,
    c_command::{CCommand, CommandSource},
    c_event::EventSink,
    c_health::Health,
};

pub fn request_commands_by_event(
    mut query: Query<(&mut CommandSource, &mut EventSink, &mut Client, &Health)>,
) {
    for (mut command_receiver, mut event_sender, mut client_connection, health) in &mut query {
        if health.val == 0 {
            continue;
        }

        let mut queue: Vec<CCommand> = Vec::new();

        for event in event_sender.queue.iter() {
            info!("{:?}", event);
            queue.append(&mut client_connection.client.request_commands_by_event(event));
            info!("{:?}", queue);
        }
        event_sender.queue.clear();
        command_receiver.queue.splice(0..0, queue);
    }
}
