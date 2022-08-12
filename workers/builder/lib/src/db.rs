use postgres::{Client, NoTls};

pub fn get_client() -> Client {
    Client::connect("host=localhost user=postgres", NoTls).unwrap()
}

pub fn upload_log(client: &mut Client, url: &str, status: &str, log: &str) {
    client
        .execute(
            r#"
            UPDATE tanks
            SET status = $2, log = $3
            WHERE url = $1;           
        "#,
            &[&url, &status, &log],
        )
        .unwrap();
}
