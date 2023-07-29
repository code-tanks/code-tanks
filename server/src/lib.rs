pub mod db;

use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::prelude::*;
use futures::stream::StreamExt;
use std::process::Command;
use std::env;

use db::*;
use r2d2_postgres::{postgres::NoTls, r2d2::PooledConnection, PostgresConnectionManager};

pub fn create_build_queue() {
    let output_raw = Command::new("curl")
        .arg("-H")
        .arg("content-type: application/json")
        .arg("-XPUT")
        .arg("-d")
        .arg(r#"{"timeout": "10m"}"#)
        .arg(format!("{}/queue/build", env::var("OCYPOD_URL").unwrap().parse::<String>().unwrap()))
        .output()
        .expect("failed to communicate with ocypod");

    let result_raw = String::from_utf8_lossy(&output_raw.stdout);
    let err_raw = String::from_utf8_lossy(&output_raw.stderr);

    println!("stdout:");
    println!("{}\n", result_raw);
    println!("stderr:");
    println!("{}\n", err_raw);
}

pub fn create_sim_queue() {
    // curl -H "content-type: application/json" -XPUT -d '{"timeout": "10m"}' ocypod:8023/queue/simulator
    let output_raw = Command::new("curl")
        .arg("-H")
        .arg("content-type: application/json")
        .arg("-XPUT")
        .arg("-d")
        .arg(r#"{"timeout": "10m"}"#)
        .arg(format!("{}/queue/simulator", env::var("OCYPOD_URL").unwrap().parse::<String>().unwrap()))
        .output()
        .expect("failed to communicate with ocypod");
    let result_raw = String::from_utf8_lossy(&output_raw.stdout);
    let err_raw = String::from_utf8_lossy(&output_raw.stderr);

    println!("stdout:");
    println!("{}\n", result_raw);
    println!("stderr:");
    println!("{}\n", err_raw);
}
pub struct HttpServer {
    pub port: u16,
}

impl HttpServer {
    pub async fn run(&mut self) {

        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port))
            .await
            .unwrap();

        listener
            .incoming()
            .for_each_concurrent(/* limit */ None, |tcpstream| async move {
                let tcpstream = tcpstream.unwrap();
                // let pool = pool.clone();
                let pool = get_db_pool();

                let mut db = pool.get().unwrap();

                handle_connection(tcpstream, &mut db).await;
            })
            .await;
        // {
        //     let stream = stream.unwrap();

        //     let pool = pool.clone();

        //     thread::scope(|s| {
        //         s.spawn(|| {
        //             let mut db = pool.get().unwrap();
        //             handle_connection(stream, &mut db);
        //         });
        //     });
        // }
    }
}

struct Path {}
impl Path {
    pub const ROOT: &str = "";
    pub const PING: &str = "ping";
    pub const UPLOAD: &str = "upload";
    pub const LOG: &str = "log";
    pub const RAW: &str = "raw";
    pub const RUN: &str = "run";
    pub const SIM: &str = "sim";
    pub const SIM_LOG: &str = "sim_log";
    pub const RECENT: &str = "recent";
}

struct Method {}
impl Method {
    pub const POST: &str = "POST";
    pub const GET: &str = "GET";
}

struct ContentType {}
impl ContentType {
    pub const JSON: &str = "application/json";
    pub const TEXT: &str = "text/plain";
}

pub struct StatusLine {}
impl StatusLine {
    pub const OK: &str = "HTTP/1.1 200 OK";
    pub const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND";
}
pub struct Response<'a> {
    pub status_line: &'a str,
    pub content: &'a str,
}
impl Response<'_> {
    pub const ROOT_RESPONSE: Response<'static> = Response {
        status_line: StatusLine::OK,
        content: "\"hello\"",
    };

    pub const PING_RESPONSE: Response<'static> = Response {
        status_line: StatusLine::OK,
        content: "\"pong\"",
    };

    pub const NOT_FOUND_RESPONSE: Response<'static> = Response {
        status_line: StatusLine::NOT_FOUND,
        content: "\"404\"",
    };

    pub const ERROR_TOO_LARGE: Response<'static> = Response {
        status_line: StatusLine::NOT_FOUND,
        content: "\"FILE TOO LARGE\"",
    };

    pub const ERROR_TOO_MANY_PLAYERS: Response<'static> = Response {
        status_line: StatusLine::NOT_FOUND,
        content: "\"TOO MANY PLAYERS\"",
    };
}

const HEADER_PADDING: usize = 150;
const MAX_BYTES_READ: usize = 1000000;
const BUFFER_SIZE_BYTES: usize = MAX_BYTES_READ + HEADER_PADDING;
const MAX_NUMBER_PLAYERS: usize = 4;

async fn handle_connection(
    mut stream: TcpStream,
    db: &mut PooledConnection<PostgresConnectionManager<NoTls>>,
) {
    let mut buffer = [0; BUFFER_SIZE_BYTES];
    let bytes_read = stream.read(&mut buffer).await.unwrap();

    println!("bytes read: {}", bytes_read);

    let request = String::from_utf8(buffer.to_vec()).unwrap();
    let method = get_header_from_request(&request);
    let path = &get_path_from_request(&request)[1..];
    let args: Vec<&str> = path.split('/').collect();
    let path = args[0];
    let args = &args[1..];

    // println!("{} {} {} {:?}", request, method, path, args);

    let url: String;
    let res_code: String;
    let res_log: String;
    let mut string_build = "could not run simulation\n".to_string();

    let mut content_type = ContentType::JSON;

    let response: Response = match (method, path) {
        (Method::GET, Path::ROOT) => Response::ROOT_RESPONSE,
        (Method::GET, Path::PING) => Response::PING_RESPONSE,
        (Method::POST, Path::UPLOAD) => {
            let uploaded_code = get_data_from_request(&request);

            let code_bytes_len = uploaded_code.bytes().len();
            println!("code size {}", code_bytes_len);

            if code_bytes_len > MAX_BYTES_READ {
                Response::ERROR_TOO_LARGE
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
                        let code: String = existing[0].get(1);

                        if code == uploaded_code {
                            break;
                        } else {
                            println!("regenerating");
                            post_fix_count += 1;
                        }
                    }
                }
                let post_fix = POST_FIX_CHAR.repeat(post_fix_count);

                let existing = get_existing(db, uploaded_code, post_fix);

                url = existing[0].get(0);
                let language: String = existing[0].get(4);

                println!("found short url {}", url);

                if needs_generation {
                    add_build_job(&format!("{},{}", url, language));
                }

                Response {
                    status_line: StatusLine::OK,
                    content: &url,
                }
            }
        }
        (Method::GET, Path::LOG) => {
            let mut res = Response::NOT_FOUND_RESPONSE;

            // handle error

            let matches = get_tank_by_hash(db, args[0]);

            if !matches.is_empty() {
                res_log = matches[0].get(2);
                res = Response {
                    status_line: StatusLine::OK,
                    content: &res_log,
                };
            }
            res
        }
        (Method::GET, Path::RAW) => {
            let mut res = Response::NOT_FOUND_RESPONSE;

            // handle error

            let matches = get_tank_by_hash(db, args[0]);

            if !matches.is_empty() {
                res_code = matches[0].get(1);

                res = Response {
                    status_line: StatusLine::OK,
                    content: &res_code,
                };
            }
            res
        }
        (Method::POST, Path::RUN) => {
            let mut res = Response::NOT_FOUND_RESPONSE;

            let data = get_data_from_request(&request);
            let tank_urls = data.split(' ').collect::<Vec<&str>>();

            if tank_urls.len() > MAX_NUMBER_PLAYERS {
                res = Response::ERROR_TOO_MANY_PLAYERS
            } else {
                let invalid_tanks = tank_urls
                    .iter()
                    .map(|f| (f.to_string(), get_tank_build_status_by_url(db, f)))
                    .filter(|g| g.1 != TankBuildStatus::Valid)
                    .collect::<Vec<(String, TankBuildStatus)>>();

                if !invalid_tanks.is_empty() {
                    for (tank_url, status) in invalid_tanks {
                        let status_str = match status {
                            TankBuildStatus::Invalid => "build failed",
                            TankBuildStatus::Building => "waiting to build",
                            TankBuildStatus::Missing => "missing",
                            _ => "",
                        };
                        string_build = string_build + &tank_url + " -> " + status_str + "\n";
                    }
                    res = Response {
                        status_line: StatusLine::OK,
                        content: &string_build,
                    };
                } else {
                    let game_url = &tank_urls.join("-");

                    println!("run: {}", game_url);

                    let mut matches = get_simulation_by_url(db, game_url);
                    if matches.is_empty() {
                        add_sim_job(&data);
                        upsert_simulation_by_url(db, game_url);
                        matches = get_simulation_by_url(db, game_url);
                    }

                    if !matches.is_empty() {
                        res_code = matches[0].get(1);

                        res = Response {
                            status_line: StatusLine::OK,
                            content: &res_code,
                        };
                    }
                }
            }

            res
        }
        (Method::GET, Path::SIM) => {
            let mut res = Response::NOT_FOUND_RESPONSE;

            println!("get sim: {:?}", args);

            // handle error

            let matches = get_simulation_by_url(db, &args.join("-"));

            if !matches.is_empty() {
                res_code = matches[0].get(1);
                content_type = ContentType::TEXT;

                res = Response {
                    status_line: StatusLine::OK,
                    content: &res_code,
                };
            }
            res
        }
        (Method::GET, Path::SIM_LOG) => {
            let mut res = Response::NOT_FOUND_RESPONSE;

            println!("get sim_log: {:?}", args);

            // handle error

            let matches = get_simulation_log_by_id(db, args[0]);

            if !matches.is_empty() {
                let out: String = matches[0].get(1);
                let err: String = matches[0].get(2);
                res_code = format!("{}\n{}", out, err);

                content_type = ContentType::TEXT;

                res = Response {
                    status_line: StatusLine::OK,
                    content: &res_code,
                };
            }

            res
        }
        (Method::GET, Path::RECENT) => {
            let mut res = Response::NOT_FOUND_RESPONSE;

            println!("get recent: {:?}", args);

            let recent = get_recent_simulations(db);

            if !recent.is_empty() {
                res_code = recent[0].get(0);
                println!("recent: {}", res_code);
                res = Response {
                    status_line: StatusLine::OK,
                    content: &res_code,
                };
            }

            res
        }
        _ => Response::NOT_FOUND_RESPONSE,
    };

    let response_string = format!(
        "{}\r\nContent-Length: {}\r\nContent-Type: {}\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET\r\ncharset=UTF-8\r\n\r\n{}",
        response.status_line,
        response.content.len(),
        content_type,
        response.content
    );

    stream.write_all(response_string.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

fn get_header_from_request(request: &str) -> &str {
    let mut splits = request.split(' ');
    splits.next().unwrap()
}

fn get_path_from_request(request: &str) -> &str {
    let mut splits = request.split(' ');
    splits.nth(1).unwrap()
}

fn get_data_from_request(request: &str) -> String {
    let mut response = "".to_string();
    let mut data_found = false;
    for line in request.lines() {
        if data_found {
            if response.is_empty() {
                response = line.to_string();
            } else if !line.starts_with('\0') {
                response = format!("{}\n{}", response, line)
            }
        }
        if line.is_empty() {
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
        .arg(format!("{}/queue/build/job", env::var("OCYPOD_URL").unwrap().parse::<String>().unwrap()))
        .output()
        .expect("failed to communicate with ocypod");

    let result_raw = String::from_utf8_lossy(&output_raw.stdout);
    let err_raw = String::from_utf8_lossy(&output_raw.stderr);

    println!("stdout:");
    println!("{}\n", result_raw);
    println!("stderr:");
    println!("{}\n", err_raw);
}

pub fn add_sim_job(url: &str) {
    // curl -i -H 'content-type: application/json' -XPOST -d '{"input": [1,2,3]}' localhost:8023/queue/demo/job
    let output_raw = Command::new("curl")
        .arg("-H")
        .arg("content-type: application/json")
        .arg("-XPOST")
        .arg("-d")
        .arg(format!(r#"{{"input": "{}"}}"#, url))
        .arg(format!("{}/queue/simulator/job", env::var("OCYPOD_URL").unwrap().parse::<String>().unwrap()))
        .output()
        .expect("failed to communicate with ocypod");

    let result_raw = String::from_utf8_lossy(&output_raw.stdout);
    let err_raw = String::from_utf8_lossy(&output_raw.stderr);

    println!("stdout:");
    println!("{}\n", result_raw);
    println!("stderr:");
    println!("{}\n", err_raw);
}

fn get_tank_build_status_by_url(
    db: &mut PooledConnection<PostgresConnectionManager<NoTls>>,
    url: &str,
) -> TankBuildStatus {
    let matches = get_tank_by_hash(db, url);

    let mut status = TankBuildStatus::Invalid;

    if !matches.is_empty() {
        let log: String = matches[0].get(2);
        let successful: bool = matches[0].get(3);

        if log == "waiting to build" {
            status = TankBuildStatus::Building;
        } else if successful {
            status = TankBuildStatus::Valid;
        }
    } else {
        status = TankBuildStatus::Missing;
    }

    status
}

#[derive(PartialEq)]
enum TankBuildStatus {
    Valid,
    Invalid,
    Building,
    Missing,
}
