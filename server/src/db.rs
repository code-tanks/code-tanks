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

        CREATE OR REPLACE FUNCTION base36_encode( long_number bigint ) 
        RETURNS text
        AS $BODY$
        /*
        * base36_encode()
        *
        * This function accepts a small or big number (base 10) and reduces its length into a string
        * that is URI-safe using the lower case 26-letter English alphabet 
        * as well as the numbers 0 - 9. The result is returned as a text string.
        *
        */
        declare
            k_base        constant integer := 36;
            k_alphabet    constant text[] := string_to_array( '0123456789abcdefghijklmnopqrstuvwxyz'::text, null);
            
            v_return_text text := '';
            v_remainder   integer;
            v_interim	  bigint;
        begin
        
            v_interim := abs( long_number );  -- Negative Numbers (sign) are ignored
        
            --Conversion Loop
            loop
        
                v_remainder     := v_interim % k_base;
            v_interim       := v_interim / k_base;
            v_return_text   := ''|| k_alphabet[ (v_remainder + 1) ] || v_return_text ;
        
            exit when v_interim <= 0;
        
            end loop ;
        
        
            return v_return_text;
        
        end;$BODY$
        LANGUAGE plpgsql
        immutable		    /* Makes no changes to data in tables */
        returns null ON NULL INPUT  /* Don't bother to call if the value is NULL */
        SECURITY INVOKER            /* No reason to use DEFINER for security */
        cost 5                      /* A made up number. Any advice? */
        ;

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
                    SUBSTRING(base36_encode(('x'||lpad(id,16,'0'))::bit(64)::bigint), 0, 8), 
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

// pub fn get_code(
//     client: &mut PooledConnection<PostgresConnectionManager<NoTls>>,
//     url: &str,
// ) -> Vec<Row> {
//     client
//         .query(
//             "
//                 SELECT code FROM tanks
//                 WHERE url = $1;
//             ",
//             &[&url],
//         )
//         .unwrap()
// }

// pub fn get_log(
//     client: &mut PooledConnection<PostgresConnectionManager<NoTls>>,
//     url: &str,
// ) -> Vec<Row> {
//     client
//         .query(
//             "
//                 SELECT log FROM tanks
//                 WHERE url = $1;
//             ",
//             &[&url],
//         )
//         .unwrap()
// }

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
