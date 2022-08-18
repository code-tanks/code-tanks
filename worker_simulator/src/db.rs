use std::env;

use postgres::{Client, NoTls};

pub fn get_client() -> Client {
    Client::connect(&env::var("DB_URL").unwrap(), NoTls).unwrap()
}

pub fn upload_sim(client: &mut Client, game_id: &str, sim: &str, successful: bool) -> bool {
    client
        .execute(
            r#"
            UPDATE simulations
            SET log = $2, successful = $3
            WHERE id = $1;           
        "#,
            &[&game_id, &sim, &successful],
        )
        .is_ok()
}
