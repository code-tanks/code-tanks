use std::{thread, time};

use ctbuilderlib::{
    db::{get_client, upload_log},
    *,
};

fn main() {
    println!("Started ctbuilder");

    // build("hello", "world");

    if !get_queues().contains(&"build".to_string()) {
        create_build_queue();
    }

    let mut client = get_client();

    loop {
        let job = get_job();

        if job.is_ok() {
            let job = job.unwrap();

            let url = job["input"].to_string();

            let lang = get_lang(&url);

            println!("{:?}", job);
            println!("");

            let build_info = build(&url, &lang);
            upload_log(&mut client, &url, &build_info.log);
        }

        thread::sleep(time::Duration::from_millis(1000));
    }
}
