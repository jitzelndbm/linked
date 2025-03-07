use askama::Template;
use std::io::Write;

use axum::{
    extract::rejection::QueryRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error as ThisError;
use tracing::{error, info};

pub type Result<T> = ::core::result::Result<T, Error>;

/// This enum contains all errors that are possible with
#[derive(Debug, ThisError)]
#[non_exhaustive]
pub enum Error {
    #[error("Page could not be found")]
    PageNotFound,

    // 400 BAD_REQUEST
    #[error("The username '{0}' could not be found in the htpasswd file")]
    UserNotFound(String),
    #[error("The provided url '{0}' is in an invalid format")]
    InvalidUrlProvided(String),
    #[error("The provided query is invalid")]
    InvalidQuery,

    // 401 UNAUTHORIZED
    #[error("A user tried to authenticate without a session")]
    SessionNotFound,

    // 500 INTERNAL_SERVER_ERROR
    #[error("An error occurred while trying to retrieve session")]
    SessionRetrieval,
    #[error("An error occurred while trying to remove a session value")]
    SessionRemoval,
    #[error("Failed to parse the htpasswd file")]
    Htpasswd,
    #[error("The htpasswd file could not be found")]
    HtpasswdFileNotFound,
    #[error("The storage directory could not be created because: {0}")]
    CouldNotCreateStoreDir(String),
    #[error("Either the config could not be found, or there is something wrong with the config")]
    ConfigError,
    #[error("The log_level field in the config is not in the right format")]
    ConfigLogFormat,
    #[error("The $HOME environment variable is not defined")]
    HomeEnvNotFound,
    #[error("The webserver could not be bound to the given ip and port")]
    BindError,
    #[error("Failed to connect to the database: {0}")]
    DatabaseConnect(String),
    #[error("A query failed: {0}")]
    QueryException(String),
}

/// This resembles the html template of an error page
#[derive(Template)]
#[template(path = "error.html", whitespace = "suppress")]
pub struct ErrorTemplate<'a> {
    status_code: &'a str,
    reason: Option<&'static str>,
    message: Option<String>,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status_code, message): (StatusCode, Option<String>) = match self {
            Self::UserNotFound(_) | Self::InvalidUrlProvided(_) | Self::InvalidQuery => {
                info!("{}", self.to_string());
                (StatusCode::BAD_REQUEST, Some(self.to_string()))
            }
            Self::SessionNotFound => {
                info!("{}", self.to_string());
                (
                    StatusCode::UNAUTHORIZED,
                    Some("No session token found".to_string()),
                )
            }
            Self::PageNotFound => (StatusCode::NOT_FOUND, None),
            _ => {
                error!("{}", self.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, None)
            }
        };

        let status = status_code;

        Response::builder()
            .body(
                ErrorTemplate {
                    status_code: status.as_str(),
                    reason: status_code.canonical_reason(),
                    message,
                }
                .render()
                .expect("Error template failed to render")
                .into(),
            )
            .expect("Error template failed to render")
    }
}

pub async fn not_found_handler() -> Response {
    Error::PageNotFound.into_response()
}

pub fn default_error_handler(error: &Error, output: &mut dyn Write) {
    writeln!(output, "[error]: {}", error).ok();
}

impl From<QueryRejection> for Error {
    fn from(_: QueryRejection) -> Self {
        Self::InvalidQuery
    }
}

///// This macro is uused to quickly implement a method to convert errors into errors that the program
///// can use to display html error pages so the `?` operator can be used
//macro_rules! impl_error {
//    ($error:ty, $transform:expr) => {
//        impl From<$error> for Error {
//            fn from(value: $error) -> Self {
//                error!("{:?}", value);
//                $transform
//            }
//        }
//    };
//}
//
//
