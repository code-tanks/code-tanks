use ctserver::HttpServer;

fn main() {
    println!("Running CodeTanks Server");

    let mut server = HttpServer { port: 8088 };
    server.run();
}
