use std::env;

use postgres::{Client, NoTls};

pub fn get_client() -> Client {
    Client::connect(&env::var("DB_URL").unwrap(), NoTls).unwrap()
}

pub fn upload_log(client: &mut Client, url: &str, log: &str) -> bool {
    client
        .execute(
            r#"
            UPDATE tanks
            SET log = $2
            WHERE url = $1;           
        "#,
            &[&url, &log],
        )
        .is_ok()
}
