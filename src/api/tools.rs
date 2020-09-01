use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use crate::api::error::ServerError;
use std::backtrace::Backtrace;

pub trait ServerUnwrap {
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

pub trait ServerUnwrapError {
    type Item;
    fn with_error(self) -> Result<Self::Item, ServerError>;
}

impl<T, E: 'static + std::error::Error> ServerUnwrapError for Result<T, E> {
    type Item = T;

    fn with_error(self) -> Result<Self::Item, ServerError> {
        self.map_err(|err| { new_internal_error!(err) })
    }
}

pub fn ok_response<B: Into<actix_web::body::Body>>(body: B) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK).body(body))
}

pub fn err_response(err: ServerError) -> actix_web::Result<HttpResponse> {
    Err(actix_web::Error::from(err))
}

#[macro_export]
macro_rules! unwrap {
    ($e:expr) => {
        ServerUnwrap::without_error($e, ||{new_internal_error!()})
    };
    ($e:expr, $err:expr) => {
        ServerUnwrap::without_error($e, ||{$err})
    };
}

#[macro_export]
macro_rules! unwrap_err {
    ($e:expr) => {
        ServerUnwrapError::with_error($e)
    };
}
