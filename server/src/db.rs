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
        CREATE EXTENSION IF NOT EXISTS "pgcrypto";

        CREATE TABLE IF NOT EXISTS tanks (
            hash        VARCHAR NOT NULL,
            code        VARCHAR NOT NULL,
            log         VARCHAR NOT NULL,
            successful  BOOL NOT NULL,
            language    VARCHAR NOT NULL,
            timestamp   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP(0)
        );

        CREATE TABLE IF NOT EXISTS simulations (
            game_url    VARCHAR PRIMARY KEY,
            log         VARCHAR NOT NULL,
            successful  BOOL NOT NULL,
            timestamp   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP(0)
        );

        CREATE TABLE IF NOT EXISTS runs (
            container_name  VARCHAR PRIMARY KEY,
            out             VARCHAR NOT NULL,
            err             VARCHAR NOT NULL,
            timestamp       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP(0)
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
                INSERT INTO tanks (hash, code, log, successful, language)
                VALUES (
                    SUBSTRING(ENCODE(DIGEST($1 || $2,'sha256'), 'hex'), 0, 8), 
                    $1, 
                    'waiting to build',
                    false,
                    $3
                );
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
                SELECT *
                FROM tanks
                WHERE tanks.hash = SUBSTRING(ENCODE(DIGEST($1 || $2,'sha256'), 'hex'), 0, 8);
            ",
            &[&code, &post_fix],
        )
        .unwrap()
}

pub fn get_tank_by_hash(
    client: &mut PooledConnection<PostgresConnectionManager<NoTls>>,
    tank_hash: &str,
) -> Vec<Row> {
    client
        .query(
            "
                SELECT * FROM tanks
                WHERE hash = $1
            ",
            &[&tank_hash],
        )
        .unwrap()
}

pub fn get_simulation_by_url(
    client: &mut PooledConnection<PostgresConnectionManager<NoTls>>,
    game_url: &str,
) -> Vec<Row> {
    client
        .query(
            "
                SELECT * FROM simulations
                WHERE game_url = $1
            ",
            &[&game_url],
        )
        .unwrap()
}

pub fn upsert_simulation_by_url(
    client: &mut PooledConnection<PostgresConnectionManager<NoTls>>,
    game_url: &str,
) {
    client
        .execute(
            "
                INSERT INTO simulations (game_url, log, successful)
                VALUES ($1, 'waiting to build', false)
                ON CONFLICT (game_url) DO NOTHING;
            ",
            &[&game_url],
        )
        .unwrap();
}

pub fn get_simulation_log_by_id(
    client: &mut PooledConnection<PostgresConnectionManager<NoTls>>,
    tank_container_name: &str,
) -> Vec<Row> {
    client
        .query(
            "
                SELECT * FROM runs
                WHERE container_name = $1
            ",
            &[&tank_container_name],
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
                        game_url,
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
