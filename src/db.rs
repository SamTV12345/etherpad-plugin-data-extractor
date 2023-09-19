use std::env;
use diesel::{Connection, PgConnection};
use crate::constants::{DATABASE_URL};

pub fn establish_connection() -> PgConnection {
    let database_url = &get_database_url();
    PgConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_database_url()->String{
    env::var(DATABASE_URL).unwrap()
}