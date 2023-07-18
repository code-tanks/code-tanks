use ctserver::{db::*, *};
use futures::future::join_all;
use rocket::State;
use rocket_db_pools::sqlx::{Pool, Postgres, Row};
// use r2d2_postgres::{postgres::NoTls, r2d2::PooledConnection, PostgresConnectionManager};

const HEADER_PADDING: usize = 150;
const MAX_BYTES_READ: usize = 1000000;
const BUFFER_SIZE_BYTES: usize = MAX_BYTES_READ + HEADER_PADDING;
const MAX_NUMBER_PLAYERS: usize = 4;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[post("/upload/<lang>", format = "text/plain", data = "<uploaded_code>")]
async fn upload(pool: &State<Pool<Postgres>>, uploaded_code: String, lang: String) -> String {
    // let uploaded_code = get_data_from_request(&request);

    let code_bytes_len = uploaded_code.bytes().len();
    println!("code size {}", code_bytes_len);

    // if code_bytes_len > MAX_BYTES_READ {
    //     Response::ERROR_TOO_LARGE
    // } else {
    const POST_FIX_CHAR: &str = "0";
    let mut post_fix_count = 0;
    let mut needs_generation = false;

    loop {
        let post_fix = POST_FIX_CHAR.repeat(post_fix_count);
        let existing = get_existing(pool, uploaded_code.to_string(), post_fix.to_string()).await;

        if existing.is_empty() {
            println!("generating short url...");
            println!("{}", uploaded_code);
            insert_tank(
                pool,
                uploaded_code.to_string(),
                post_fix.to_string(),
                lang.to_string(),
            ).await;
            needs_generation = true;
        } else {
            let code: String = existing[0].get(2);

            if code == uploaded_code {
                break;
            } else {
                println!("regenerating");
                post_fix_count += 1;
            }
        }
    }
    let post_fix = POST_FIX_CHAR.repeat(post_fix_count);

    let existing = get_existing(pool, uploaded_code, post_fix).await;
    let url = existing[0].get::<String, _>("url");
    let language: String = existing[0].get(5);

    println!("found short url {}", url);

    if needs_generation {
        add_build_job(&format!("{},{}", url, language));
    }

    url
    // }
}

#[get("/log/<url>")]
async fn log(pool: &State<Pool<Postgres>>, url: String) -> String {
    // let mut res = "NOT_FOUND_RESPONSE";

    // handle error

    let matches = get_tank_by_url(pool, &url).await;
    if !matches.is_empty() {
        matches[0].get::<String, _>("log")
    } else {
        "NOT_FOUND_RESPONSE".to_string()
    }
}

#[get("/raw/<url>")]
async fn raw(pool: &State<Pool<Postgres>>, url: String) -> String {
    // let mut res = /

    // handle error

    let matches = get_tank_by_url(pool, &url).await;

    if !matches.is_empty() {
        matches[0].get::<String, _>("code")
    } else {
        "NOT_FOUND_RESPONSE".to_string()
    }
}

async fn b(pool: &Pool<Postgres>, f: &&str) -> (String, TankBuildStatus) {
    (f.to_string(), get_tank_build_status_by_url(pool, f).await)
}

#[get("/run/<data>")]
async fn run(pool: &State<Pool<Postgres>>, data: String) -> String {
    let mut res = "NOT_FOUND_RESPONSE";

    // let data = get_data_from_request(&request);
    let tank_urls = data.split(' ').collect::<Vec<&str>>();

    if tank_urls.len() > MAX_NUMBER_PLAYERS {
        return "ERROR_TOO_MANY_PLAYERS".to_string();
    } else {
        let invalid_tanks = join_all(tank_urls.iter().map(|f| async { b(pool, f).await }))
            .await
            .into_iter()
            .filter(|g| g.1 != TankBuildStatus::Valid)
            .collect::<Vec<(String, TankBuildStatus)>>();

        if !invalid_tanks.is_empty() {
            let mut string_build = "".to_string();
            for (tank_url, status) in invalid_tanks {
                let status_str = match status {
                    TankBuildStatus::Invalid => "build failed",
                    TankBuildStatus::Building => "waiting to build",
                    TankBuildStatus::Missing => "missing",
                    _ => "",
                };
                string_build = string_build + &tank_url + " -> " + status_str + "\n";
            }
            return string_build;
        } else {
            let game_id = &tank_urls.join("-");

            println!("run: {}", game_id);

            let mut matches = get_simulation_by_url(pool, game_id).await;
            if matches.is_empty() {
                add_sim_job(&data);
                upsert_simulation_by_url(pool, game_id).await;
                matches = get_simulation_by_url(pool, game_id).await;
            }

            if !matches.is_empty() {
                return matches[0].get(1);
            }
        }
    }

    res.to_string()
}

#[get("/sim/<data>")]
async fn sim(pool: &State<Pool<Postgres>>, data: String) -> String {
    let mut res = "NOT_FOUND_RESPONSE";
    let args = data.split(" ").collect::<Vec<&str>>();

    println!("get sim: {:?}", args);

    // handle error

    let matches = get_simulation_by_url(pool, &args.join("-")).await;

    if !matches.is_empty() {
        return matches[0].get(1);
    }
    res.to_string()
}

#[get("/sim_log/<data>")]
async fn sim_log(pool: &State<Pool<Postgres>>, data: String) -> String {
    let mut res = "NOT_FOUND_RESPONSE";
    let args = data.split(" ").collect::<Vec<&str>>();

    println!("get sim_log: {:?}", args);

    // handle error

    let matches = get_simulation_log_by_id(pool, args[0]).await;

    if !matches.is_empty() {
        let out: String = matches[0].get(1);
        let err: String = matches[0].get(2);
        return format!("{}\n{}", out, err);
    }

    res.to_string()
}

#[get("/recent")]
async fn recent(pool: &State<Pool<Postgres>>) -> String {
    let mut res = "NOT_FOUND_RESPONSE";

    println!("get recent");

    let recent = get_recent_simulations(pool).await;

    if !recent.is_empty() {
        return recent[0].get(0);
    }

    res.to_string()
}

#[launch]
async fn rocket() -> _ {
    println!("Running CodeTanks Server");
    create_build_queue();
    create_sim_queue();

    rocket::build().manage(get_db_pool().await).mount(
        "/",
        routes![index, ping, upload, log, raw, run, sim, sim_log, recent],
    )
} // pub const PING: &str = "ping";
  // pub const UPLOAD: &str = "upload";
  // pub const LOG: &str = "log";
  // pub const RAW: &str = "raw";
  // pub const RUN: &str = "run";
  // pub const SIM: &str = "sim";
  // pub const SIM_LOG: &str = "sim_log";
  // pub const RECENT: &str = "recent";

// #[async_std::main]
// async fn main() {
//     println!("Running CodeTanks Server");
//     create_build_queue();
//     create_sim_queue();

//     let mut server = HttpServer { port: 8088 };
//     server.run().await;
// }
