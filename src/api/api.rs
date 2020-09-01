use actix_web::{web, HttpResponse};
use crate::database::user_db::UserDB;
use crate::model::user::User;
use crate::model::session::Session;
use crate::database::session_db::SessionDB;
use crate::database::todo_db::{TodoDB};
use crate::model::session_request::{SessionRequest, AddTodoRequest, ToggleTodoRequest, RemoveTodoRequest};
use crate::model::todo::TodoItem;
use crate::api::error::ServerError;
use crate::api::tools::{err_response, ok_response, ServerUnwrap, ServerUnwrapError};
use std::backtrace::Backtrace;

#[post("/regi")]
pub async fn regi(user_db: web::Data<UserDB>, todo_db: web::Data<TodoDB>, user: web::Json<User>) -> actix_web::Result<HttpResponse> {
    if user_db.find(&user.name).is_some() {
        return err_response(ServerError::UserExists);
    }

    let user = unwrap!(user_db.add(user.0))?;
    todo_db.regi_user(user);
    ok_response("")
}

#[post("/login")]
pub async fn login(user_db: web::Data<UserDB>, session_db: web::Data<SessionDB>, user: web::Json<User>) -> actix_web::Result<HttpResponse> {
    let user = unwrap!(user_db.find(&user.name), ServerError::UserDoesntExist)?;
    let session = Session::new(user);
    let response = session.id.to_string();
    session_db.add(session);
    ok_response(response)
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

#[post("/add")]
pub async fn add(session_db: web::Data<SessionDB>,
                 todo_db: web::Data<TodoDB>,
                 request: web::Json<AddTodoRequest>,
) -> actix_web::Result<HttpResponse> {
    let session = unwrap!(session_db.find(&request.session_id), ServerError::InvalidSession)?;
    let user = unwrap!(session.user.upgrade())?;

    let todo_item = TodoItem::new(&*request.todo_name);
    let response = todo_item.id.to_string();
    unwrap_err!(todo_db.add_todo(&user, todo_item))?;
    ok_response(response)
}

#[post("/toggle")]
pub async fn toggle(session_db: web::Data<SessionDB>,
                    todo_db: web::Data<TodoDB>,
                    request: web::Json<ToggleTodoRequest>,
) -> actix_web::Result<HttpResponse> {
    let session = unwrap!(session_db.find(&request.session_id), ServerError::InvalidSession)?;
    let user = unwrap!(session.user.upgrade())?;

    unwrap_err!(todo_db.toggle_todo(&user, request.todo_id,request.completed))?;
    ok_response("")
}

#[post("/remove")]
pub async fn remove(session_db: web::Data<SessionDB>,
                    todo_db: web::Data<TodoDB>,
                    request: web::Json<RemoveTodoRequest>,
) -> actix_web::Result<HttpResponse> {
    let session = unwrap!(session_db.find(&request.session_id), ServerError::InvalidSession)?;
    let user = unwrap!(session.user.upgrade())?;

    unwrap_err!(todo_db.remove_todo(&user, request.todo_id))?;
    ok_response("")
}
