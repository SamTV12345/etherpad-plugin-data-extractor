use std::env;
use diesel::{Connection, SqliteConnection};
use crate::constants::{DATABASE_URL, DATABASE_URL_DEFAULT_SQLITE};

pub fn establish_connection() -> SqliteConnection {
    let database_url = &get_database_url();
    SqliteConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_database_url()->String{
    env::var(DATABASE_URL).unwrap_or(DATABASE_URL_DEFAULT_SQLITE.to_string())
}