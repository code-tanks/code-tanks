use std::env;

use postgres::{Client, NoTls};

pub fn get_client() -> Client {
    Client::connect(&env::var("DB_URL").unwrap(), NoTls).unwrap()
}

pub fn upload_sim(client: &mut Client, game_url: &str, sim: &str, successful: bool) -> bool {
    client
        .execute(
            r#"
            UPDATE simulations
            SET log = $2, successful = $3
            WHERE game_url = $1;           
        "#,
            &[&game_url, &sim, &successful],
        )
        .is_ok()
}

pub fn upload_log_to_db(client: &mut Client, tank_container_name: &str, out: &str, err: &str) -> bool {
    client
        .execute(
            r#"
            INSERT INTO runs (container_name, out, err)
            VALUES($1, $2, $3)
            ON CONFLICT (container_name) DO NOTHING;        
        "#,
            &[&tank_container_name, &out, &err],
        )
        .is_ok()
}
