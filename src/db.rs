use std::env;
use diesel::{Connection};
use diesel::prelude::SqliteConnection;
use crate::constants::{DATABASE_URL};

pub fn establish_connection() -> SqliteConnection {
    let database_url = &get_database_url();
    SqliteConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_database_url()->String{
    env::var(DATABASE_URL).unwrap()
}