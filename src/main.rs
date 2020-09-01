#![feature(in_band_lifetimes)]
#![feature(backtrace)]

mod api;
mod database;
mod model;

#[macro_use]
extern crate actix_web;
extern crate serde;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate uuid;

use std::env;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use crate::api::{regi, login, list, add, toggle, remove};
use crate::database::user_db::UserDB;
use crate::database::session_db::SessionDB;
use crate::database::todo_db::TodoDB;

const SERVER_ADDRESS: &str = "0.0.0.0:8001";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug,actix_web=debug,actix_server=info");
    env_logger::init();

    let user_db = web::Data::new(UserDB::new());
    let session_db = web::Data::new(SessionDB::new());
    let todo_db = web::Data::new(TodoDB::new());

    HttpServer::new(move || {
        App::new()
            .app_data(user_db.clone())
            .app_data(session_db.clone())
            .app_data(todo_db.clone())
            .service(regi)
            .service(login)
            .service(list)
            .service(add)
            .service(toggle)
            .service(remove)
    })
        .bind(SERVER_ADDRESS)?
        .run()
        .await?;

    //todo save db
    Ok(())
}
