#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

pub mod schema;
pub mod models;

pub fn get_database_url() -> String {
    dotenv().expect("Unable to load .env file.");
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn establish_connection() -> PgConnection {
    let database_url = get_database_url();

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}