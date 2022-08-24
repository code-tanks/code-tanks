use std::env;

use postgres::{Client, NoTls};

pub fn get_client() -> Client {
    Client::connect(&env::var("DB_URL").unwrap(), NoTls).unwrap()
}

pub fn upload_log(client: &mut Client, url: &str, log: &str, successful: bool) -> bool {
    client
        .execute(
            r#"
            UPDATE tanks
            SET log = $2, successful = $3
            WHERE url = $1;           
        "#,
            &[&url, &log, &(successful as i8)],
        )
        .is_ok()
}
