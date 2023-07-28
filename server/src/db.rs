use std::env;

use r2d2_postgres::{
    postgres::{NoTls, Row},
    r2d2::{Pool, PooledConnection},
    PostgresConnectionManager,
};

pub fn get_db_pool() -> Pool<PostgresConnectionManager<NoTls>> {
    let manager =
        PostgresConnectionManager::new(env::var("DB_URL").unwrap().parse().unwrap(), NoTls);
    let pool = Pool::new(manager).unwrap();
    let mut client = pool.get().unwrap();
    client.batch_execute(r#"
        /* https://gist.github.com/david-sanabria/0d3ff67eb56d2750502aed4186d6a4a7 */
        CREATE EXTENSION IF NOT EXISTS "pgcrypto";

        /* completed|cancelled|timed_out|queued|running|failed */

        CREATE TABLE IF NOT EXISTS tanks (
            id          TEXT PRIMARY KEY,
            url         VARCHAR NOT NULL,
            code        VARCHAR NOT NULL,
            log         VARCHAR NOT NULL,
            successful  BOOL NOT NULL,
            language    VARCHAR NOT NULL,
            timestamp   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP(0)
        );

        CREATE TABLE IF NOT EXISTS simulations (
            id          VARCHAR PRIMARY KEY,
            log         VARCHAR NOT NULL,
            successful  BOOL NOT NULL,
            timestamp   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP(0)
        );

        CREATE TABLE IF NOT EXISTS runs (
            id          VARCHAR PRIMARY KEY,
            out         VARCHAR NOT NULL,
            err         VARCHAR NOT NULL,
            timestamp   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP(0)
        );
    "#).unwrap();

    pool
}

pub fn insert_tank(
    client: &mut PooledConnection<PostgresConnectionManager<NoTls>>,
    code: String,
    post_fix: String,
    language: String,
) {
    client
        .execute(
            r#"
                WITH cte AS (
                    SELECT ENCODE(DIGEST($1 || $2,'sha256'), 'hex') AS id
                )
                INSERT INTO tanks (id, url, code, log, successful, language)
                SELECT 
                    id, 
                    SUBSTRING(id, 0, 8), 
                    $1, 
                    'waiting to build',
                    false,
                    $3
                FROM cte;
            "#,
            &[&code, &post_fix, &language],
        )
        .unwrap();
}

pub fn get_existing(
    client: &mut PooledConnection<PostgresConnectionManager<NoTls>>,
    code: String,
    post_fix: String,
) -> Vec<Row> {
    client
        .query(
            "
                WITH cte AS (
                    SELECT ENCODE(DIGEST($1 || $2,'sha256'), 'hex') AS id
                ), matches AS (
                    SELECT * FROM tanks, cte
                    WHERE tanks.id = cte.id
                )
                SELECT *
                FROM matches;
            ",
            &[&code, &post_fix],
        )
        .unwrap()
}

pub fn get_tank_by_url(
    client: &mut PooledConnection<PostgresConnectionManager<NoTls>>,
    url: &str,
) -> Vec<Row> {
    client
        .query(
            "
                SELECT * FROM tanks
                WHERE url = $1
            ",
            &[&url],
        )
        .unwrap()
}

pub fn get_simulation_by_url(
    client: &mut PooledConnection<PostgresConnectionManager<NoTls>>,
    url: &str,
) -> Vec<Row> {
    client
        .query(
            "
                SELECT * FROM simulations
                WHERE id = $1
            ",
            &[&url],
        )
        .unwrap()
}

pub fn upsert_simulation_by_url(
    client: &mut PooledConnection<PostgresConnectionManager<NoTls>>,
    url: &str,
) {
    client
        .execute(
            "
                INSERT INTO simulations (id, log, successful)
                VALUES ($1, 'waiting to build', false)
                ON CONFLICT (id) DO NOTHING;
            ",
            &[&url],
        )
        .unwrap();
}

pub fn get_simulation_log_by_id(
    client: &mut PooledConnection<PostgresConnectionManager<NoTls>>,
    id: &str,
) -> Vec<Row> {
    client
        .query(
            "
                SELECT * FROM runs
                WHERE id = $1
            ",
            &[&id],
        )
        .unwrap()
}

pub fn get_recent_simulations(
    client: &mut PooledConnection<PostgresConnectionManager<NoTls>>,
) -> Vec<Row> {
    client
        .query(
            "
                SELECT json_agg(to_json(r))::varchar
                FROM (
                    SELECT
                        id,
                        timestamp,
                        SPLIT_PART(log, E'\n', -1)::json as results,
                        SPLIT_PART(log, E'\n', -1)::json->'tanks' as tanks,
                        SPLIT_PART(log, E'\n', -1)::json->'winner' as winner
                    FROM simulations
                    WHERE log != 'waiting to build'
                    ORDER BY timestamp DESC
                    LIMIT 10
                ) r
            ",
            &[],
        )
        .unwrap()
}
