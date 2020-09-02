use actix_web::HttpResponse;
use actix_web::http::StatusCode;

#[get("/")]
pub async fn webpage() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../frontend/inlined/index.html")))
}
