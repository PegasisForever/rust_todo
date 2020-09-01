use std::backtrace::Backtrace;
use serde::export::Formatter;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;

#[derive(Debug)]
pub enum ServerError {
    InvalidSession,
    UserExists,
    UserDoesntExist,
    InternalError {
        error: Option<Box<dyn std::error::Error>>,
        backtrace: Backtrace,
    },
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::InternalError { backtrace, error } => {
                if let Some(error) = error {
                    write!(f, "{:?}\n{}", error, backtrace)
                } else {
                    write!(f, "Unspecified error\n{}", backtrace)
                }
            }
            err => write!(f, "{:?}", err),
        }
    }
}

impl actix_web::error::ResponseError for ServerError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServerError::InvalidSession | ServerError::UserDoesntExist => StatusCode::FORBIDDEN,
            ServerError::UserExists => StatusCode::CONFLICT,
            ServerError::InternalError { error: _, backtrace: _ } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::new(self.status_code())
    }
}

#[macro_export]
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
