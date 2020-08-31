mod userdb;

#[macro_use]
extern crate actix_web;

use actix_web::{web, App, HttpServer, HttpResponse};
use actix_web::http::{StatusCode};
use userdb::{UserDB, User};

//TODO proper error handling
#[post("/regi")]
async fn regi(db: web::Data<UserDB>, user: web::Json<User>) -> actix_web::Result<HttpResponse> {
    match db.add(user.0) {
        Ok(_) => {
            Ok(HttpResponse::build(StatusCode::OK).body(""))
        }
        Err(message) => {
            Ok(HttpResponse::build(StatusCode::BAD_REQUEST).body(message))
        }
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let user_db = web::Data::new(UserDB::get());

    HttpServer::new(move || {
        App::new()
            .app_data(user_db.clone())
            .service(regi)
    })
        .bind("0.0.0.0:8001")?
        .run()
        .await
}
