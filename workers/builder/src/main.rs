use std::{thread, time};

use ctbuilderlib::*;

enum Langs {}

impl Langs {
    const DART: &'static str = "dart";
}

fn get_lang(url: &str) -> &'static str {
    Langs::DART
}

fn main() {
    println!("Started ctbuilder");

    build("hello", "world");

    if !get_queues().contains(&"build".to_string()) {
        create_build_queue();
    }

    loop {
        let job = get_job();

        if job.is_ok() {
            let job = job.unwrap();

            let url = job["input"].to_string();

            let lang = get_lang(&url);

            println!("{:?}", job);
            println!("");

            let build_info = build(&url, &lang);
        }

        thread::sleep(time::Duration::from_millis(1000));
    }
}
