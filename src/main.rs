use std::thread;
use actix_web::{App, HttpServer};
use clokwerk::{Scheduler, TimeUnits};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use crate::download_service::download_current_plugins;
use crate::package_controller::get_packages;

mod db;
mod constants;
mod package_controller;
mod schema;
mod entities;
mod download_service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut conn = db::establish_connection();
    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
    let res_migration = conn.run_pending_migrations(MIGRATIONS);

    if res_migration.is_err(){
        println!("Error running migrations: {:?}", res_migration);
    }

    thread::spawn(|| {
        download_current_plugins();

        /*let mut scheduler = Scheduler::new();
        scheduler.every(1.day()).run(|| {
            println!("Running daily download");
            download_current_plugins();
        });*/
    }
    );

    HttpServer::new(move||{
        App::new()
            .service(get_packages)
    })
        .workers(4)
        .bind(("0.0.0.0",9000))?
        .run()
        .await
}
