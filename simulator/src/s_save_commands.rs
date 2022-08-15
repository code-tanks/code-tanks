use bevy_ecs::prelude::Query;
use std::{fs::OpenOptions, io::Write};

use crate::c_command::CommandSource;

pub fn save_commands(query: Query<&CommandSource>) {
    for command_receiver in &query {
        let grouped_commands = command_receiver.queue[0];

        println!("save_commands {:?}", grouped_commands);

        let mut f = OpenOptions::new()
            .append(true)
            .open("./sim.txt")
            .expect("Unable to open file");
        f.write_all(format!("{}\n", grouped_commands).to_string().as_bytes())
            .expect("Unable to write data");

        println!("commands remaining {:?}", command_receiver.queue);
    }
}
