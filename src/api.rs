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
use std::backtrace::Backtrace;
use std::error::Error;

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
    InternalError {
        error: Option<Box<dyn std::error::Error>>,
        backtrace: Backtrace,
    },
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::InvalidSession => write!(f, "{:?}", ServerError::InvalidSession),
            ServerError::InternalError { backtrace, error } => {
                if let Some(error) = error {
                    write!(f, "{:?}\n{}", error, backtrace)
                } else {
                    write!(f, "Unspecified error\n{}", backtrace)
                }
            }
        }
    }
}

impl actix_web::error::ResponseError for ServerError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServerError::InvalidSession => StatusCode::UNAUTHORIZED,
            ServerError::InternalError { error: _, backtrace: _ } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::new(self.status_code())
    }
}

macro_rules! new_internal_error {
    () => {{
        let err = ServerError::InternalError{
            backtrace: Backtrace::capture(),
            error: None,
        };
        error!("{}", &err);
        err
    }};
    ($expression:expr) => {{
        let err = ServerError::InternalError{
            backtrace: Backtrace::capture(),
            error: Some(Box::new($expression)),
        };
        error!("{}", &err);
        err
    }};
}

macro_rules! unwrap {
    ($e:expr) => {
        ServerUnwrap::without_error($e, ||{new_internal_error!()})
    };
    ($e:expr, $err:expr) => {
        ServerUnwrap::without_error($e, ||{$err})
    };
}

trait ServerUnwrap {
    type Item;
    fn without_error<F: FnOnce() -> ServerError>(self, error_fn: F) -> Result<Self::Item, ServerError>;
}

impl<T> ServerUnwrap for Option<T> {
    type Item = T;

    fn without_error<F: FnOnce() -> ServerError>(self, error_fn: F) -> Result<Self::Item, ServerError> {
        self.ok_or_else(error_fn)
    }
}

impl<T, E> ServerUnwrap for Result<T, E> {
    type Item = T;

    fn without_error<F: FnOnce() -> ServerError>(self, error_fn: F) -> Result<Self::Item, ServerError> {
        self.map_err(|_| { error_fn() })
    }
}

macro_rules! unwrap_err {
    ($e:expr) => {
        ServerUnwrapError::with_error($e)
    };
}

trait ServerUnwrapError {
    type Item;
    fn with_error(self) -> Result<Self::Item, ServerError>;
}

impl<T, E: 'static + std::error::Error> ServerUnwrapError for Result<T, E> {
    type Item = T;

    fn with_error(self) -> Result<Self::Item, ServerError> {
        self.map_err(|err| { new_internal_error!(err) })
    }
}

fn ok_response<B: Into<actix_web::body::Body>>(body: B) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK).body(body))
}

#[post("/list")]
pub async fn list(session_db: web::Data<SessionDB>,
                  todo_db: web::Data<TodoDB>,
                  request: web::Json<SessionRequest>,
) -> actix_web::Result<HttpResponse> {
    let session = unwrap!(session_db.find(&request.session_id), ServerError::InvalidSession)?;
    let user = unwrap!(session.user.upgrade())?;

    let all_todo_list = unwrap!(todo_db.list.lock())?;
    let todo_list = unwrap!(all_todo_list.get(&user))?;

    ok_response(format!("{:?}", todo_list))
}

pub async fn _add(session_db: web::Data<SessionDB>,
                  todo_db: web::Data<TodoDB>,
                  request: web::Json<AddTodoRequest>,
) -> Result<String, ServerError> {
    let session = unwrap!(session_db.find(&request.session_id), ServerError::InvalidSession)?;
    let user = unwrap!(session.user.upgrade())?;

    let todo_item = TodoItem::new(&*request.todo_name);
    let response = todo_item.id.to_string();
    unwrap_err!(todo_db.add_todo(&user, todo_item))?;
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
        Err(ServerError::InternalError { backtrace, error }) => {
            if let Some(error) = error {
                println!("{:?}", error);
            } else {
                println!("Unspecified error")
            }
            println!("{}", backtrace);
            Err(error::ErrorInternalServerError(""))
        }
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
