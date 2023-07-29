use std::env;

use postgres::{Client, NoTls};

pub fn get_client() -> Client {
    Client::connect(&env::var("DB_URL").unwrap(), NoTls).unwrap()
}

pub fn upload_log(client: &mut Client, hash: &str, log: &str, successful: bool) -> bool {
    client
        .execute(
            r#"
            UPDATE tanks
            SET log = $2, successful = $3
            WHERE hash = $1;           
        "#,
            &[&hash, &log, &successful],
        )
        .is_ok()
}
