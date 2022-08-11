use std::{thread, time};

use ctbuilderlib::*;

fn main() {
    println!("Started ctbuilder");

    if !get_queues().contains(&"build".to_string()) {
        create_build_queue();
    }

    loop {
        let job = get_job();

        if job.is_ok() {
            let job = job.unwrap();

            println!("{:?}", job);
            println!("");
        }

        thread::sleep(time::Duration::from_millis(1000));
    }
}
