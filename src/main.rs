mod api;
mod database;
mod model;

#[macro_use]
extern crate actix_web;

extern crate serde;

extern crate uuid;

use actix_web::{web, App, HttpServer};
use crate::api::{regi, login, list};
use crate::database::user_db::UserDB;
use crate::database::session_db::SessionDB;
use crate::database::todo_db::TodoDB;

const SERVER_ADDRESS: &str = "0.0.0.0:8001";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://{}.", SERVER_ADDRESS);

    let user_db = web::Data::new(UserDB::get());
    let session_db = web::Data::new(SessionDB::get());
    let todo_db = web::Data::new(TodoDB::get());

    HttpServer::new(move || {
        App::new()
            .app_data(user_db.clone())
            .app_data(session_db.clone())
            .app_data(todo_db.clone())
            .service(regi)
            .service(login)
            .service(list)
    })
        .bind(SERVER_ADDRESS)?
        .run()
        .await?;

    //todo save db
    println!("Server exited.");
    Ok(())
}
