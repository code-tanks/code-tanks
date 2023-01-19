pub mod db;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::thread;

use db::*;
use r2d2_postgres::{postgres::NoTls, r2d2::PooledConnection, PostgresConnectionManager};

use crate::responses::{Response, StatusLines};
pub fn create_build_queue() {
    let output_raw = Command::new("curl")
        .arg("-H")
        .arg("content-type: application/json")
        .arg("-XPUT")
        .arg("-d")
        .arg(r#"{"timeout": "10m"}"#)
        .arg("ocypod:8023/queue/build")
        .output()
        .expect("failed to communicate with ocypod");

    let result_raw = String::from_utf8_lossy(&output_raw.stdout);
    let err_raw = String::from_utf8_lossy(&output_raw.stderr);

    println!("stdout:");
    println!("{}", result_raw.to_string());
    println!("");
    println!("stderr:");
    println!("{}", err_raw.to_string());
    println!("");
}

pub fn create_sim_queue() {
    // curl -H "content-type: application/json" -XPUT -d '{"timeout": "10m"}' ocypod:8023/queue/simulator
    let output_raw = Command::new("curl")
        .arg("-H")
        .arg("content-type: application/json")
        .arg("-XPUT")
        .arg("-d")
        .arg(r#"{"timeout": "10m"}"#)
        .arg("ocypod:8023/queue/simulator")
        .output()
        .expect("failed to communicate with ocypod");
    let result_raw = String::from_utf8_lossy(&output_raw.stdout);
    let err_raw = String::from_utf8_lossy(&output_raw.stderr);

    println!("stdout:");
    println!("{}", result_raw.to_string());
    println!("");
    println!("stderr:");
    println!("{}", err_raw.to_string());
    println!("");
}
pub struct HttpServer {
    pub port: u16,
}

impl HttpServer {
    pub fn run(&mut self) {
        let pool = get_db_pool();

        for stream in TcpListener::bind(format!("0.0.0.0:{}", self.port))
            .unwrap()
            .incoming()
        {
            let stream = stream.unwrap();

            let pool = pool.clone();

            thread::scope(|s| {
                s.spawn(|| {
                    let mut db = pool.get().unwrap();
                    handle_connection(stream, &mut db);
                });
            });
        }
    }
}
// enum Paths {}
mod paths {
    pub const ROOT: &'static str = "";
    pub const PING: &'static str = "ping";
    pub const UPLOAD: &'static str = "upload";
    pub const LOG: &'static str = "log";
    pub const RAW: &'static str = "raw";
    pub const RUN: &'static str = "run";
    pub const SIM: &'static str = "sim";
    pub const SIM_LOG: &'static str = "sim_log";
}

// enum Methods {}
mod methods {
    pub const POST: &'static str = "POST";
    pub const GET: &'static str = "GET";
}

mod content_types {
    pub const JSON: &'static str = "application/json";
    pub const TEXT: &'static str = "text/plain";
}

// enum Responses {}
mod responses {
    type StatusLine<'a> = &'a str;

    pub enum StatusLines {}

    impl StatusLines {
        pub const OK: StatusLine<'static> = "HTTP/1.1 200 OK";
        pub const NOT_FOUND: StatusLine<'static> = "HTTP/1.1 404 NOT FOUND";
    }
    pub struct Response<'a> {
        pub status_line: StatusLine<'a>,
        pub content: &'a str,
    }
    pub const ROOT_RESPONSE: Response<'static> = Response {
        status_line: StatusLines::OK,
        content: "\"hello\"",
    };

    pub const PING_RESPONSE: Response<'static> = Response {
        status_line: StatusLines::OK,
        content: "\"pong\"",
    };

    pub const NOT_FOUND_RESPONSE: Response<'static> = Response {
        status_line: StatusLines::NOT_FOUND,
        content: "\"404\"",
    };

    pub const ERROR_TOO_LARGE: Response<'static> = Response {
        status_line: StatusLines::NOT_FOUND,
        content: "\"FILE TOO LARGE\"",
    };

    pub const ERROR_TOO_MANY_PLAYERS: Response<'static> = Response {
        status_line: StatusLines::NOT_FOUND,
        content: "\"TOO MANY PLAYERS\"",
    };
}

const HEADER_PADDING: usize = 150;
const MAX_BYTES_READ: usize = 1000000;
const BUFFER_SIZE_BYTES: usize = MAX_BYTES_READ + HEADER_PADDING;
const MAX_NUMBER_PLAYERS: usize = 4;

fn handle_connection(
    mut stream: TcpStream,
    db: &mut PooledConnection<PostgresConnectionManager<NoTls>>,
) {
    let mut buffer = [0; BUFFER_SIZE_BYTES];
    let bytes_read = stream.read(&mut buffer).unwrap();

    println!("bytes read: {}", bytes_read);

    let request = String::from_utf8(buffer.to_vec()).unwrap();
    let method = get_header_from_request(&request);
    let path = &get_path_from_request(&request)[1..];
    let args: Vec<&str> = path.split("/").collect();
    let path = args[0];
    let args = &args[1..];

    println!("{} {} {} {:?}", request, method, path, args);

    let url: String;
    let res_code: String;
    let res_log: String;
    let mut string_build = "could not run simulation\n".to_string();

    let mut content_type = content_types::JSON;

    let response: Response = match (method, path) {
        (methods::GET, paths::ROOT) => responses::ROOT_RESPONSE,
        (methods::GET, paths::PING) => responses::PING_RESPONSE,
        (methods::POST, paths::UPLOAD) => {
            let uploaded_code = get_data_from_request(&request);

            let code_bytes_len = uploaded_code.bytes().len();
            println!("code size {}", code_bytes_len);

            if code_bytes_len > MAX_BYTES_READ {
                responses::ERROR_TOO_LARGE
            } else {
                const POST_FIX_CHAR: &str = "0";
                let mut post_fix_count = 0;
                let mut needs_generation = false;

                loop {
                    let post_fix = POST_FIX_CHAR.repeat(post_fix_count);
                    let existing =
                        get_existing(db, uploaded_code.to_string(), post_fix.to_string());

                    if existing.is_empty() {
                        println!("generating short url...");
                        insert_tank(
                            db,
                            uploaded_code.to_string(),
                            post_fix.to_string(),
                            args[0].to_string(),
                        );
                        needs_generation = true;
                    } else {
                        let code: String = existing[0].get(2);

                        if code == uploaded_code {
                            break;
                        } else {
                            println!("regenerating");
                            post_fix_count = post_fix_count + 1;
                        }
                    }
                }
                let post_fix = POST_FIX_CHAR.repeat(post_fix_count);

                let existing = get_existing(db, uploaded_code.to_string(), post_fix.to_string());

                url = existing[0].get(1);
                let language: String = existing[0].get(5);

                println!("found short url {}", url);

                if needs_generation {
                    add_build_job(&format!("{},{}", url, language));
                }

                Response {
                    status_line: StatusLines::OK,
                    content: &url,
                }
            }
        }
        (methods::GET, paths::LOG) => {
            let mut res = responses::NOT_FOUND_RESPONSE;

            // handle error

            let matches = get_tank_by_url(db, args[0]);

            if !matches.is_empty() {
                res_log = matches[0].get(3);
                res = Response {
                    status_line: StatusLines::OK,
                    content: &res_log,
                };
            }
            res
        }
        (methods::GET, paths::RAW) => {
            let mut res = responses::NOT_FOUND_RESPONSE;

            // handle error

            let matches = get_tank_by_url(db, args[0]);

            if !matches.is_empty() {
                res_code = matches[0].get(2);

                res = Response {
                    status_line: StatusLines::OK,
                    content: &res_code,
                };
            }
            res
        }
        (methods::POST, paths::RUN) => {
            let mut res = responses::NOT_FOUND_RESPONSE;

            let data = get_data_from_request(&request);
            let tank_urls = data.split(" ").collect::<Vec<&str>>();

            if tank_urls.len() > MAX_NUMBER_PLAYERS {
                res = responses::ERROR_TOO_MANY_PLAYERS
            } else {
                let invalid_tanks = tank_urls
                    .iter()
                    .map(|f| (f.to_string(), get_tank_build_status_by_url(db, f)))
                    .filter(|g| g.1 != TankBuildStatus::VALID)
                    .collect::<Vec<(String, TankBuildStatus)>>();

                if !invalid_tanks.is_empty() {
                    for (tank_url, status) in invalid_tanks {
                        let status_str = match status {
                            TankBuildStatus::INVALID => "build failed",
                            TankBuildStatus::BUILDING => "waiting to build",
                            TankBuildStatus::MISSING => "missing",
                            _ => "",
                        };
                        string_build = string_build + &tank_url + " -> " + status_str + "\n";
                    }
                    res = Response {
                        status_line: StatusLines::OK,
                        content: &string_build,
                    };
                } else {
                    let game_id = &tank_urls.join("-");

                    println!("run: {}", game_id);

                    let mut matches = get_simulation_by_url(db, game_id);
                    if matches.is_empty() {
                        add_sim_job(&data);
                        upsert_simulation_by_url(db, game_id);
                        matches = get_simulation_by_url(db, game_id);
                    }

                    if !matches.is_empty() {
                        res_code = matches[0].get(1);

                        res = Response {
                            status_line: StatusLines::OK,
                            content: &res_code,
                        };
                    }
                }
            }

            res
        }
        (methods::GET, paths::SIM) => {
            let mut res = responses::NOT_FOUND_RESPONSE;

            println!("get sim: {:?}", args);

            // handle error

            let matches = get_simulation_by_url(db, &args.join("-"));

            if !matches.is_empty() {
                res_code = matches[0].get(1);

                res = Response {
                    status_line: StatusLines::OK,
                    content: &res_code,
                };
            }
            res
        }
        (methods::GET, paths::SIM_LOG) => {
            let mut res = responses::NOT_FOUND_RESPONSE;

            println!("get sim_log: {:?}", args);

            // handle error

            let matches = get_simulation_log_by_id(db, &args[0]);

            if !matches.is_empty() {
                let out: String = matches[0].get(1);
                let err: String = matches[0].get(2);
                res_code = format!("{}\n{}", out, err);

                content_type = content_types::TEXT;

                res = Response {
                    status_line: StatusLines::OK,
                    content: &res_code,
                };
            }

            res
        }
        _ => responses::NOT_FOUND_RESPONSE,
    };

    let response_string = format!(
        "{}\r\nContent-Length: {}\r\nContent-Type: {}\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET\r\ncharset=UTF-8\r\n\r\n{}",
        response.status_line,
        response.content.len(),
        content_type,
        response.content
    );

    stream.write(response_string.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn get_header_from_request(request: &String) -> &str {
    let mut splits = request.split(" ");
    splits.nth(0).unwrap()
}

fn get_path_from_request(request: &String) -> &str {
    let mut splits = request.split(" ");
    splits.nth(1).unwrap()
}

fn get_data_from_request(request: &String) -> String {
    let mut response = "".to_string();
    let mut data_found = false;
    for line in request.lines() {
        if data_found {
            if response.len() == 0 {
                response = format!("{}", line)
            } else if !line.starts_with('\0') {
                response = format!("{}\n{}", response, line)
            }
        }
        if line.len() == 0 {
            data_found = true
        };
    }
    response.trim_matches(char::from(0)).to_string()
}

pub fn add_build_job(input: &str) {
    // curl -i -H 'content-type: application/json' -XPOST -d '{"input": [1,2,3]}' localhost:8023/queue/demo/job
    let output_raw = Command::new("curl")
        .arg("-H")
        .arg("content-type: application/json")
        .arg("-XPOST")
        .arg("-d")
        .arg(format!(r#"{{"input": "{}"}}"#, input))
        .arg("ocypod:8023/queue/build/job")
        .output()
        .expect("failed to communicate with ocypod");

    let result_raw = String::from_utf8_lossy(&output_raw.stdout);
    let err_raw = String::from_utf8_lossy(&output_raw.stderr);

    println!("stdout:");
    println!("{}", result_raw.to_string());
    println!("");
    println!("stderr:");
    println!("{}", err_raw.to_string());
    println!("");
}

pub fn add_sim_job(url: &str) {
    // curl -i -H 'content-type: application/json' -XPOST -d '{"input": [1,2,3]}' localhost:8023/queue/demo/job
    let output_raw = Command::new("curl")
        .arg("-H")
        .arg("content-type: application/json")
        .arg("-XPOST")
        .arg("-d")
        .arg(format!(r#"{{"input": "{}"}}"#, url))
        .arg("ocypod:8023/queue/simulator/job")
        .output()
        .expect("failed to communicate with ocypod");

    let result_raw = String::from_utf8_lossy(&output_raw.stdout);
    let err_raw = String::from_utf8_lossy(&output_raw.stderr);

    println!("stdout:");
    println!("{}", result_raw.to_string());
    println!("");
    println!("stderr:");
    println!("{}", err_raw.to_string());
    println!("");
}

fn get_tank_build_status_by_url(
    db: &mut PooledConnection<PostgresConnectionManager<NoTls>>,
    url: &str,
) -> TankBuildStatus {
    let matches = get_tank_by_url(db, url);

    let mut status = TankBuildStatus::INVALID;

    if !matches.is_empty() {
        let log: String = matches[0].get(3);
        let successful: bool = matches[0].get(4);

        if log == "waiting to build" {
            status = TankBuildStatus::BUILDING;
        } else if successful {
            status = TankBuildStatus::VALID;
        }
    } else {
        status = TankBuildStatus::MISSING;
    }

    status
}

#[derive(PartialEq)]
enum TankBuildStatus {
    VALID,
    INVALID,
    BUILDING,
    MISSING,
}
