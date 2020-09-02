#![feature(in_band_lifetimes)]
#![feature(backtrace)]

mod api;
mod database;
mod model;
mod tools;

#[macro_use]
extern crate actix_web;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate uuid;

use std::env;
use actix_web::{web, App, HttpServer};
use crate::api::api::{regi, login, list, add, toggle, remove};
use crate::database::user_db::UserDB;
use crate::database::session_db::SessionDB;
use crate::database::todo_db::TodoDB;
use actix_web::middleware::Logger;

const SERVER_ADDRESS: &str = "0.0.0.0:8001";
const USER_DB_PATH: &str = "data/user_db.json";
const TODO_DB_PATH: &str = "data/todo_db.json";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug,actix_server=info");
    env_logger::init();

    let user_db = web::Data::new(UserDB::new(String::from(USER_DB_PATH)));
    let user_db_global = user_db.clone();
    let session_db = web::Data::new(SessionDB::new());
    let todo_db = web::Data::new(TodoDB::new(String::from(TODO_DB_PATH), user_db.get_ref()));
    let todo_db_global = todo_db.clone();

    HttpServer::new(move || {
        App::new()
            .app_data(user_db.clone())
            .app_data(session_db.clone())
            .app_data(todo_db.clone())
            .wrap(Logger::default())
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

    user_db_global.save();
    todo_db_global.save();
    //todo save db
    Ok(())
}
