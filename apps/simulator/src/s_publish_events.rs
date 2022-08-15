use bevy_ecs::system::Query;

use crate::{
    c_client::Client,
    c_command::{Command, CommandSource},
    c_event::EventSink,
};

pub fn publish_events(mut query: Query<(&mut CommandSource, &EventSink, &Client)>) {
    for (mut command_receiver, event_sender, client_connection) in &mut query {
        let mut queue: Vec<Command> = Vec::new();
        for event in event_sender.queue.iter() {
            queue.append(&mut client_connection.client.request_commands_by_event(event));
        }
        command_receiver.queue.splice(0..0, queue);
    }
}
