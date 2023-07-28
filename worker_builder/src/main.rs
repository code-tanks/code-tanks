use std::{thread, time};

use worker_builder::{
    build, create_build_queue,
    db::{get_client, upload_log},
    get_build_job, push_to_registry, remove_image, update_build_job,
};

fn main() {
    println!("Started ctbuilder");

    create_build_queue();

    let mut client = get_client();

    loop {
        // println!("getting build job");
        let job = get_build_job();

        if !job.is_empty() {
            println!("got {:?}", job);
            let id = &job[0];
            let input = &job[1]
                .split(',')
                .map(|f| f.to_string())
                .collect::<Vec<String>>();
            let url = &input[0];
            let lang = &input[1];

            println!("id: {:?}, {}, {}\n", job, url, lang);

            let build_log = build(url, lang);
            let pushed_to_registry = push_to_registry(url);
            let uploaded_log = upload_log(&mut client, url, &build_log, pushed_to_registry);
            remove_image(url);

            update_build_job(id, uploaded_log && pushed_to_registry);
        }
        // println!("no jobs found. sleeping for 1 second");

        thread::sleep(time::Duration::from_millis(1000));
    }
}
