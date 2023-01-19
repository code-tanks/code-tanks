use bevy::prelude::*;
use ct_api::CCommand;

use crate::{c_client::Client, c_command::CommandSource, c_event::EventSink, c_health::Health};

pub fn request_commands_by_event(
    mut query: Query<(
        Entity,
        &mut CommandSource,
        &mut EventSink,
        &mut Client,
        &Health,
    )>,
) {
    for (entity, mut command_receiver, mut event_sender, mut client_connection, health) in
        &mut query
    {
        if health.val == 0 || event_sender.queue.is_empty() {
            continue;
        }

        let mut queue: Vec<CCommand> = Vec::new();

        for event in event_sender.queue.iter() {
            println!("{:?} {:?}", entity, event);
            let mut new_commands = client_connection.client.request_commands_by_event(event);
            // println!("request_commands_by_event {:?} {:?}", entity, new_commands);
            if !new_commands.is_empty() {
                queue.append(&mut new_commands);
            }
        }
        event_sender.queue.clear();
        command_receiver.queue.splice(0..0, queue);
        // println!("commands {:?} {:?}", entity, command_receiver.queue);
    }
}
