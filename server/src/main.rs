use ctserver::{create_build_queue, create_sim_queue, HttpServer};

#[async_std::main]
async fn main() {
    println!("Running CodeTanks Server");
    create_build_queue();
    create_sim_queue();

    let mut server = HttpServer { port: 8088 };
    server.run().await;
}
