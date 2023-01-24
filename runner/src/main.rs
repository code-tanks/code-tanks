use ct_api::HttpServer;

use crate::my_tank::*;

pub mod my_tank;

fn main() {
    println!("Hello, world!");
    let mut server = HttpServer { port: 8080 };
    server.run(&mut create_tank());
}
