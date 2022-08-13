use std::{thread, time};

use ctbuilder::{
    build, create_build_queue,
    db::{get_client, upload_log},
    get_job, get_lang, get_queues, push_to_registry, remove_image, update_job,
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

            let url = job["input"].as_str().unwrap();
            let id: u64 = job["id"].as_u64().unwrap();

            let lang = get_lang(&url);

            println!("{:?}", job);
            println!("");

            let build_info = build(&url, &lang);
            upload_log(&mut client, &url, &build_info.log);
            push_to_registry(&url);
            remove_image(&url);
            update_job(id, build_info.successful);
        }

        thread::sleep(time::Duration::from_millis(1000));
    }
}
