mod api;
mod database;
mod model;

#[macro_use]
extern crate actix_web;

use actix_web::{web, App, HttpServer};
use crate::api::{regi, login};
use crate::database::userdb::UserDB;

const SERVER_ADDRESS: &str = "0.0.0.0:8001";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://{}.",SERVER_ADDRESS);

    let user_db = web::Data::new(UserDB::get());

    HttpServer::new(move || {
        App::new()
            .app_data(user_db.clone())
            .service(regi)
            .service(login)
    })
        .bind(SERVER_ADDRESS)?
        .run()
        .await?;

    //todo save db
    println!("Server exited.");
    Ok(())
}
