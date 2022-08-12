pub mod db;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::thread;

use db::*;
use r2d2_postgres::{postgres::NoTls, r2d2::PooledConnection, PostgresConnectionManager};
use serde_json::{from_str, json, Value};

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

// enum RequestType {
//     NotFound,
//     Root,
//     Ping,
//     Upload,
// }

// #[derive(PartialEq, Eq)]
// struct Path<'a>(&'a str);
enum Paths {}
impl Paths {
    const ROOT: &'static str = "/";
    const PING: &'static str = "/ping";
    const UPLOAD: &'static str = "/upload";
    const LOG: &'static str = "/log";
}

struct Response<'a> {
    status_line: StatusLine<'a>,
    content: &'a str,
}

type StatusLine<'a> = &'a str;

enum StatusLines {}

impl StatusLines {
    const OK: StatusLine<'static> = "HTTP/1.1 200 OK";
    const NOT_FOUND: StatusLine<'static> = "HTTP/1.1 404 NOT FOUND";
}

enum Responses {}
impl Responses {
    const ROOT_RESPONSE: Response<'static> = Response {
        status_line: StatusLines::OK,
        content: "\"hello\"",
    };

    const PING_RESPONSE: Response<'static> = Response {
        status_line: StatusLines::OK,
        content: "\"pong\"",
    };

    const NOT_FOUND_RESPONSE: Response<'static> = Response {
        status_line: StatusLines::NOT_FOUND,
        content: "\"404\"",
    };
}

fn handle_connection(
    mut stream: TcpStream,
    db: &mut PooledConnection<PostgresConnectionManager<NoTls>>,
) {
    let mut buffer = [0; 20000];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8(buffer.to_vec()).unwrap();
    let path = get_path_from_request(&request);
    // let request_type = match Path(path) {
    //     Paths::ROOT => RequestType::Root,
    //     Paths::PING => RequestType::Ping,
    //     Paths::UPLOAD => RequestType::Upload,
    //     _ => {
    //         let url = &path[1..];

    //         let contents = get_code(db, url);

    //         let mut res = RequestType::NotFound;

    //         if !contents.is_empty() {
    //             let data: String = contents[0].get(0);

    //             let y = json!(data);
    //         }

    //         res
    //     }
    // };

    let url: String;
    let code_json: Value;

    let response = match path {
        Paths::ROOT => Responses::ROOT_RESPONSE,
        Paths::PING => Responses::PING_RESPONSE,
        Paths::UPLOAD => {
            let data = get_data_from_request(&request);
            // println!("{}", data);

            let data = Value::String(data);

            // let code = data.as_str().unwrap();

            // println!("{} {}", data, data.as_str().unwrap());

            const POST_FIX_CHAR: &str = "0";
            let mut post_fix_count = 0;
            let mut needs_generation = false;

            loop {
                let post_fix = POST_FIX_CHAR.repeat(post_fix_count);
                let existing = get_existing(db, data.to_string(), post_fix.to_string());

                if existing.is_empty() {
                    println!("generating short url...");
                    insert_tank(db, data.to_string(), post_fix.to_string());
                    needs_generation = true;
                } else {
                    let code_as_json_string: String = existing[0].get(2);

                    // println!("test {} {}", code_as_json_string, data.to_string());

                    if code_as_json_string == data.to_string() {
                        break;
                    } else {
                        println!("regenerating");
                        post_fix_count = post_fix_count + 1;
                    }
                }
            }
            let post_fix = POST_FIX_CHAR.repeat(post_fix_count);

            let existing = get_existing(db, data.to_string(), post_fix.to_string());

            url = existing[0].get(1);

            println!("found short url {}", url);

            if needs_generation {
                add_build_job(&url);
            }

            Response {
                status_line: StatusLines::OK,
                content: &url,
            }
            // println!("{}", data);
        }
        _ => {
            let url = &path[1..];

            // get log for url
            println!("{}", &url[(url.len() - Paths::LOG.len())..]);
            if &url[(url.len() - Paths::LOG.len())..] == Paths::LOG {
                println!("is log request!")
            }

            println!("{}", url);

            let matches = get_code(db, url);

            let mut res = Responses::NOT_FOUND_RESPONSE;

            if !matches.is_empty() {
                let code_as_json_string: String = matches[0].get(1);

                code_json = from_str(&code_as_json_string).unwrap();

                // code = code_json.as_str().unwrap();

                res = Response {
                    status_line: StatusLines::OK,
                    content: &code_json.as_str().unwrap(),
                };
            }

            res
        }
    };
    // println!("{}, {}", path, request_type as u32);

    let response_string = format!(
        "{}\r\nContent-Length: {}\r\nContent-Type: application/json; charset=UTF-8\r\n\r\n{}",
        response.status_line,
        response.content.len(),
        response.content
    );

    stream.write(response_string.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn get_path_from_request(request: &String) -> &str {
    let mut splits = request.split(" ");
    splits.nth(1).unwrap()
}

fn get_data_from_request(request: &String) -> String {
    println!("raw data");
    println!("{}", request);

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

pub fn add_build_job(url: &str) {
    // curl -i -H 'content-type: application/json' -XPOST -d '{"input": [1,2,3]}' localhost:8023/queue/demo/job
    Command::new("curl")
        .arg("-H")
        .arg("content-type: application/json")
        .arg("-XPOST")
        .arg("-d")
        .arg(
            serde_json::json!({
                "input": url,
            })
            .to_string(),
        )
        .arg("mq:8023/queue/build/job")
        .output()
        .expect("failed to communicate with ocypod");
}
