use actix_web::{web, error, HttpResponse};
use actix_web::http::{StatusCode};
use crate::database::user_db::{UserDB, UserDBError};
use crate::model::user::User;
use crate::model::session::Session;
use crate::database::session_db::SessionDB;
use crate::database::todo_db::{TodoDBError, TodoDB};
use crate::model::session_request::SessionRequest;
use crate::model::todo::TodoItem;

#[post("/regi")]
pub async fn regi(db: web::Data<UserDB>,todo_db: web::Data<TodoDB>, user: web::Json<User>) -> actix_web::Result<HttpResponse> {
    match db.add(user.0) {
        Ok(user) => {
            todo_db.regi_user(user.clone());
            Ok(HttpResponse::build(StatusCode::OK).body(""))
        },
        Err(UserDBError::UserExists) => Err(error::ErrorConflict(UserDBError::UserExists))
    }
}

#[post("/login")]
pub async fn login(user_db: web::Data<UserDB>, session_db: web::Data<SessionDB>, user: web::Json<User>) -> actix_web::Result<HttpResponse> {
    match user_db.find(&user.name) {
        Some(found_user) => if user.password == found_user.upgrade().unwrap().password {
            let session = Session::new(found_user);
            let response = session.id.to_string();
            session_db.add(session.clone());
            Ok(HttpResponse::build(StatusCode::OK).body(response))
        } else {
            Err(error::ErrorForbidden(""))
        }
        None => Err(error::ErrorForbidden(""))
    }
}


#[post("/list")]
pub async fn list(session_db: web::Data<SessionDB>,
                  todo_db: web::Data<TodoDB>,
                  request: web::Json<SessionRequest>,
) -> actix_web::Result<HttpResponse> {
    let session_id = request.session_id;
    match session_db.find(&session_id) {
        None => Err(error::ErrorForbidden("")),
        Some(session) => if session.is_valid() {
            match session.user.upgrade() {
                None => Err(error::ErrorForbidden("")),
                Some(user) => {
                    let temp = todo_db.list.lock().unwrap();
                    let todo_list = temp
                        .get(&user)
                        .unwrap();
                    Ok(HttpResponse::build(StatusCode::OK).body(format!("{:?}", todo_list)))
                }
            }
        } else {
            Err(error::ErrorForbidden(""))
        }
    }
}
