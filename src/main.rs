#[macro_use]
extern crate actix_web;

use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse, Result};
use actix_web::http::{header, Method, StatusCode};

#[get("/regi")]
async fn welcome(req: HttpRequest) -> Result<HttpResponse> {

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("awa"))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(welcome)
    })
        .bind("0.0.0.0:8001")?
        .run()
        .await
}
