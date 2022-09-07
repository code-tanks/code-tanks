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

pub fn upload_log_to_db(client: &mut Client, tank_id: &str, out: &str, err: &str) -> bool {
    client
        .execute(
            r#"
            INSERT INTO runs (id, out, err)
            VALUES($1, $2, $3)
            ON CONFLICT (id) DO NOTHING;        
        "#,
            &[&tank_id, &out, &err],
        )
        .is_ok()
}
