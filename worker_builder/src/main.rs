use std::{thread, time};

use worker_builder::{
    build, create_build_queue,
    db::{get_client, upload_log},
    get_build_job, get_lang, push_to_registry, remove_image, update_build_job,
};

fn main() {
    println!("Started ctbuilder");

    //  build("hello", "world");

    // if !get_queues().contains(&"build".to_string()) {
    create_build_queue();
    // }

    let mut client = get_client();

    loop {
        let job = get_build_job();

        if !job.is_empty() {
            let id = &job[0];
            let url = &job[1];

            let lang = get_lang(&url);

            println!("{:?}", job);
            println!("");

            let build_info = build(&url, &lang);
            let uploaded_log = upload_log(&mut client, &url, &build_info.log);
            let pushed_to_registry = push_to_registry(&url);
            remove_image(&url);

            update_build_job(
                &id,
                build_info.successful && uploaded_log && pushed_to_registry,
            );
        }

        thread::sleep(time::Duration::from_millis(1000));
    }
}
