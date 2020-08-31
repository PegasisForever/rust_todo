use actix_web::{web, error, HttpResponse};
use actix_web::http::{StatusCode};
use crate::database::userdb::{UserDB, UserDBError};
use crate::model::user::User;

#[post("/regi")]
pub async fn regi(db: web::Data<UserDB>, user: web::Json<User>) -> actix_web::Result<HttpResponse> {
    match db.add(user.0) {
        Ok(_) => Ok(HttpResponse::build(StatusCode::OK).body("")),
        Err(UserDBError::UserExists) => Err(error::ErrorConflict(UserDBError::UserExists))
    }
}

#[post("/login")]
pub async fn login(db: web::Data<UserDB>, user: web::Json<User>) -> actix_web::Result<HttpResponse> {
    match db.find(&user.name) {
        Some(found_user) => if user.password == found_user.upgrade().unwrap().password {
            Ok(HttpResponse::build(StatusCode::OK).body("some token"))
        } else {
            Err(error::ErrorForbidden(""))
        }
        None => Err(error::ErrorForbidden(""))
    }
}
