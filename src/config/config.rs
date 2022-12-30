use std::env;
use std::env::VarError;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub url_postgres: String,
    pub url_domain: String,
}


pub fn get_config() -> Config {
    let url_postgres = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:somePassword@localhost:5432/postgres".to_string());
    println!("{}", url_postgres.to_string());
    let url_domain = env::var("URL_DOMAIN").unwrap_or_else(|_| "localhost".to_string());
    Config{url_postgres, url_domain }
        /*match (env::var("DATABASE_URL"), ) {
        (Ok(url_postgres), Ok(url_domain)) => {Ok(Config{url_postgres, url_domain})}
        (Err(error_postgres), Err(error_domain)) => {Err(VarError::NotPresent)}
        _ => {Err(VarError::NotPresent)}
    }*/
}