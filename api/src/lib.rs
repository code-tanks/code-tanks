use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use serde_json::{json, Value};

use crate::responses::{Response, StatusLines};

pub trait Tank: Send + Sync {
    fn run(&mut self, commands: &mut Vec<CCommand>);
    fn on_event(&mut self, commands: &mut Vec<CCommand>, event: &Value);
}

pub struct HttpServer {
    pub port: u16,
}

impl HttpServer {
    pub fn run(&mut self, tank: &mut dyn Tank) {
        for stream in TcpListener::bind(format!("0.0.0.0:{}", self.port))
            .unwrap()
            .incoming()
        {
            let stream = stream.unwrap();

            thread::scope(|s| {
                s.spawn(|| {
                    handle_connection(stream, tank);
                });
            });
        }
    }
}

mod paths {
    pub const ROOT: &'static str = "";
    pub const PING: &'static str = "ping";
    pub const REQUEST_COMMANDS: &'static str = "request_commands";
    pub const REQUEST_COMMANDS_BY_EVENT: &'static str = "request_commands_by_event";
}

mod methods {
    pub const POST: &'static str = "POST";
    pub const GET: &'static str = "GET";
}

mod content_types {
    pub const JSON: &'static str = "application/json";
}

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
}

fn handle_connection(mut stream: TcpStream, tank: &mut dyn Tank) {
    let mut buffer = [0; 20000];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8(buffer.to_vec()).unwrap();
    let method = get_header_from_request(&request);
    let path = &get_path_from_request(&request)[1..];
    let args: Vec<&str> = path.split("/").collect();
    let path = args[0];

    let content: String;

    let content_type = content_types::JSON;

    let commands:&mut Vec<CCommand> = &mut Vec::new();


    let response = match (method, path) {
        (methods::GET, paths::ROOT) => responses::ROOT_RESPONSE,
        (methods::GET, paths::PING) => responses::PING_RESPONSE,
        (methods::GET, paths::REQUEST_COMMANDS) => {
            tank.run(commands);
            content = json!(commands).to_string();
            commands.clear();
            Response {
                status_line: StatusLines::OK,
                content: &content,
            }
        }
        (methods::POST, paths::REQUEST_COMMANDS_BY_EVENT) => {
            let event: Value = serde_json::from_str(&get_data_from_request(&request)).unwrap();
            tank.on_event(commands, &event);
            content = json!(commands).to_string();
            commands.clear();
            Response {
                status_line: StatusLines::OK,
                content: &content,
            }
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

pub type CCommand = u64;

pub enum CCommands {}

impl CCommands {
    pub const NONE: CCommand = 0b0;
    pub const MOVE_FORWARD: CCommand = 0b1;
    pub const MOVE_BACKWARD: CCommand = 0b1 << 1;
    pub const ROTATE_TANK_CLOCKWISE: CCommand = 0b1 << 2;
    pub const ROTATE_TANK_COUNTER_CLOCKWISE: CCommand = 0b1 << 3;
    pub const FIRE: CCommand = 0b1 << 4;
    pub const ROTATE_GUN_CLOCKWISE: CCommand = 0b1 << 5;
    pub const ROTATE_GUN_COUNTER_CLOCKWISE: CCommand = 0b1 << 6;
    pub const ROTATE_RADAR_CLOCKWISE: CCommand = 0b1 << 7;
    pub const ROTATE_RADAR_COUNTER_CLOCKWISE: CCommand = 0b1 << 8;
    pub const LOCK_GUN: CCommand = 0b1 << 9;
    pub const UNLOCK_GUN: CCommand = 0b1 << 10;
    pub const LOCK_RADAR: CCommand = 0b1 << 11;
    pub const UNLOCK_RADAR: CCommand = 0b1 << 12;
    pub const SELF_DESTRUCT: CCommand = 0b1 << 13;
}
