use postgres::{Client, NoTls};

pub fn get_client() -> Client {
    Client::connect("host=localhost user=postgres", NoTls).unwrap()
}

pub fn upload_log(client: &mut Client, url: &str, log: &str) {
    client
        .execute(
            r#"
            UPDATE tanks
            SET log = $2
            WHERE url = $1;           
        "#,
            &[&url, &log],
        )
        .unwrap();
}
