mod userdb;

#[macro_use]
extern crate actix_web;

use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, Result};
use actix_web::http::{StatusCode};
use userdb::{UserDB, User};

#[get("/regi")]
async fn regi(req: HttpRequest, db: web::Data<UserDB>) -> Result<HttpResponse> {
    println!("{:?}", db.size());
    db.add(User::new("awa".into(), "Awa".into()));

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("awa"))
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
