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

        CREATE OR REPLACE FUNCTION base62_encode( long_number bigint ) 
        RETURNS text
        AS $BODY$
        /*
        * base62_encode()
        *
        * This function accepts a small or big number (base 10) and reduces its length into a string
        * that is URI-safe using the upper and lower case 26-letter English alphabet 
        * as well as the numbers 0 - 9. The result is returned as a text string that can be decoded
        * based to base10 using the base62_decode() function.
        *
        * You can find a handy explainer at https://helloacm.com/base62/
        *
        *
        * HISTORY
        * 2018-03-13 david sanabria, office of systems integration
        *            - New function
        *
        */
        declare
            k_base        constant integer := 62;
            k_alphabet    constant text[] := string_to_array( '0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz'::text, null);
            
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
        
        CREATE OR REPLACE FUNCTION base62_decode( encoded_text text ) 
        RETURNS bigint
        AS $BODY$
        /*
        * base62_decode()
        *
        * This function accepts a string that has been base62 encoded. Any characters that are not valid
        * are simply ignored, so you can safely pass a formatted string and still get a valid result like you
        * do with a UUID field.
        *
        * HISTORY
        * 2018-03-13 david sanabria, office of systems integration
        *            - New function
        *
        */
        declare
            k_base          constant integer := 62;
            k_alphabet      constant text := '0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz';
            
            v_encoded_arr   text[];
            v_return_result bigint   := 0;
            v_interim	    bigint;
            v_index         integer;  -- Pointer to input array
            v_token         text;
            v_power         integer := 0;  -- reverse pointer, used for position exponent (e.g. 2^32)
        begin
        
            -- check for guard values
            if encoded_text is null or length( encoded_text ) = 0 then
            return null;
            end if;
        
            -- reverse the input string to make the exponent math simpler below
            v_encoded_arr := string_to_array( reverse( encoded_text ) , null );
        
        
            --Conversion Loop
            foreach v_token in array v_encoded_arr
            loop
        
            v_index := strpos( k_alphabet, v_token );
        
            if v_index = 0 then
                raise notice 'Token ignored "%"', v_token;
                --ignore invalid tokens, which allows formatted strings to be processed (e.g. '{abc-1Lg}')
            else
                    v_return_result := v_return_result + ( ( v_index - 1) *  pow( k_base, v_power) );
                v_power := 1 + v_power; --increment after each valid loop
                end if;
        
            end loop;
        
        
            return v_return_result;
        
        end;$BODY$
        LANGUAGE plpgsql
        immutable		    /* Makes no changes to data in tables */
        returns null ON NULL INPUT  /* Don't bother to call if the value is NULL */
        SECURITY INVOKER            /* No reason to use DEFINER for security */
        cost 5                      /* A made up number. Any advice? */
        ;

        CREATE TABLE IF NOT EXISTS tanks (
            id          TEXT PRIMARY KEY,
            url         VARCHAR NOT NULL,
            code        VARCHAR NOT NULL,
            status      VARCHAR NOT NULL
        );

        /* completed|cancelled|timed_out|queued|running|failed */

        CREATE TABLE IF NOT EXISTS build_jobs (
            id          TEXT PRIMARY KEY,
            url         VARCHAR NOT NULL,
            code        VARCHAR NOT NULL
        );

    "#).unwrap();

    pool
}

pub fn insert_tank(
    client: &mut PooledConnection<PostgresConnectionManager<NoTls>>,
    code_as_json_string: String,
    post_fix: String,
) {
    client
        .execute(
            "
                WITH cte AS (
                    SELECT ENCODE(DIGEST($1 || $2,'sha256'), 'hex') AS id
                )
                INSERT INTO tanks (id, url, code, status)
                SELECT id, base62_encode(('x'||lpad(SUBSTRING(id, 0, 11),16,'0'))::bit(64)::bigint), $1, 'queued'
                FROM cte;
            ",
            &[&code_as_json_string, &post_fix],
        )
        .unwrap();
}

pub fn get_existing(
    client: &mut PooledConnection<PostgresConnectionManager<NoTls>>,
    code_as_json_string: String,
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
            &[&code_as_json_string, &post_fix],
        )
        .unwrap()
}

pub fn get_code(
    client: &mut PooledConnection<PostgresConnectionManager<NoTls>>,
    url: &str,
) -> Vec<Row> {
    client
        .query(
            "
                SELECT code FROM tanks
                WHERE url = $1
            ",
            &[&url],
        )
        .unwrap()
}
