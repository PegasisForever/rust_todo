use actix_web::{web, error, HttpResponse};
use actix_web::http::{StatusCode};
use crate::database::user_db::{UserDB, UserDBError};
use crate::model::user::User;
use crate::model::session::Session;
use crate::database::session_db::SessionDB;
use crate::database::todo_db::{TodoDB};
use crate::model::session_request::{SessionRequest, AddTodoRequest, ToggleTodoRequest, RemoveTodoRequest};
use crate::model::todo::TodoItem;
use serde::export::Formatter;

#[post("/regi")]
pub async fn regi(db: web::Data<UserDB>, todo_db: web::Data<TodoDB>, user: web::Json<User>) -> actix_web::Result<HttpResponse> {
    match db.add(user.0) {
        Ok(user) => {
            todo_db.regi_user(user.clone());
            Ok(HttpResponse::build(StatusCode::OK).body(""))
        }
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

#[derive(Debug)]
pub enum ServerError {
    InvalidSession,
    InternalError(Box<dyn std::error::Error>),
}

macro_rules! new_internal_error {
    ($expression:expr) => {
        ServerError::InternalError(Box::new($expression))
    };
}


#[derive(Debug)]
pub enum InternalError {
    CannotGetLock,
    CannotUpgradeUser,
    CannotFindTodoList,
}

impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for InternalError {}

pub async fn _list(session_db: web::Data<SessionDB>,
                   todo_db: web::Data<TodoDB>,
                   request: web::Json<SessionRequest>,
) -> Result<String, ServerError> {
    let session = session_db.find(&request.session_id)
        .ok_or(ServerError::InvalidSession)?;
    let user = session.user.upgrade()
        .ok_or(new_internal_error!(InternalError::CannotUpgradeUser))?;

    let all_todo_list = todo_db.list.lock()
        .map_err(|_| { new_internal_error!(InternalError::CannotGetLock) })?;
    let todo_list = all_todo_list.get(&user)
        .ok_or(new_internal_error!(InternalError::CannotFindTodoList))?;
    Ok(format!("{:?}", todo_list))
}

#[post("/list")]
pub async fn list(session_db: web::Data<SessionDB>,
                  todo_db: web::Data<TodoDB>,
                  request: web::Json<SessionRequest>,
) -> actix_web::Result<HttpResponse> {
    match _list(session_db, todo_db, request).await {
        Ok(response) => Ok(HttpResponse::build(StatusCode::OK).body(response)),
        Err(ServerError::InvalidSession) => Err(error::ErrorForbidden("")),
        Err(ServerError::InternalError(err)) => Err(error::ErrorInternalServerError(format!("{:?}", err))),
    }
}

pub async fn _add(session_db: web::Data<SessionDB>,
                  todo_db: web::Data<TodoDB>,
                  request: web::Json<AddTodoRequest>,
) -> Result<String, ServerError> {
    let session = session_db.find(&request.session_id)
        .ok_or(ServerError::InvalidSession)?;
    let user = session.user.upgrade()
        .ok_or(new_internal_error!(InternalError::CannotUpgradeUser))?;

    let todo_item = TodoItem::new(&*request.todo_name);
    let response = todo_item.id.to_string();
    todo_db.add_todo(&user, todo_item)
        .map_err(|err| { new_internal_error!(err) })?;
    Ok(response)
}

#[post("/add")]
pub async fn add(session_db: web::Data<SessionDB>,
                 todo_db: web::Data<TodoDB>,
                 request: web::Json<AddTodoRequest>,
) -> actix_web::Result<HttpResponse> {
    match _add(session_db, todo_db, request).await {
        Ok(response) => Ok(HttpResponse::build(StatusCode::OK).body(response)),
        Err(ServerError::InvalidSession) => Err(error::ErrorForbidden("")),
        Err(ServerError::InternalError(err)) => Err(error::ErrorInternalServerError(format!("{:?}", err))),
    }
}

#[post("/toggle")]
pub async fn toggle(session_db: web::Data<SessionDB>,
                    todo_db: web::Data<TodoDB>,
                    request: web::Json<ToggleTodoRequest>,
) -> actix_web::Result<HttpResponse> {
    let session_id = request.session_id;
    match session_db.find(&session_id) {
        None => Err(error::ErrorForbidden("")),
        Some(session) => if session.is_valid() {
            match session.user.upgrade() {
                None => Err(error::ErrorForbidden("")),
                Some(user) => {
                    todo_db.toggle_todo(&user, request.todo_id, request.completed);
                    Ok(HttpResponse::build(StatusCode::OK).body(""))
                }
            }
        } else {
            Err(error::ErrorForbidden(""))
        }
    }
}

#[post("/remove")]
pub async fn remove(session_db: web::Data<SessionDB>,
                    todo_db: web::Data<TodoDB>,
                    request: web::Json<RemoveTodoRequest>,
) -> actix_web::Result<HttpResponse> {
    let session_id = request.session_id;
    match session_db.find(&session_id) {
        None => Err(error::ErrorForbidden("")),
        Some(session) => if session.is_valid() {
            match session.user.upgrade() {
                None => Err(error::ErrorForbidden("")),
                Some(user) => {
                    todo_db.remove_todo(&user, request.todo_id);
                    Ok(HttpResponse::build(StatusCode::OK).body(""))
                }
            }
        } else {
            Err(error::ErrorForbidden(""))
        }
    }
}
