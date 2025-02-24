use askama::Template;
use std::io::Write;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error as ThisError;
use tracing::{error, info};

pub type Result<T> = ::core::result::Result<T, Error>;

/// This enum contains all errors that are possible with
#[derive(ThisError)]
#[non_exhaustive]
pub enum Error {
    //#[error("A generic error '{0}', with message '{1}' was sent to a client")]
    //Message(StatusCode, &'static str),
    //#[error("Generic Error '{0}' sent to a client")]
    //Status(StatusCode),
    #[error("A user tried to authenticate without a session")]
    SessionNotFound,

    // 500 Errors
    #[error("An error occurred while trying to retrieve session")]
    SessionRetrieval,
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
}

/// This resembles the html template of an error page
#[derive(Template)]
#[template(path = "error.html", whitespace = "suppress")]
pub struct ErrorTemplate<'a> {
    status_code: &'a str,
    reason: Option<&'static str>,
    message: Option<&'static str>,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status_code, message) = match self {
            Self::SessionNotFound => {
                info!("{}", self.to_string());
                (StatusCode::UNAUTHORIZED, Some("No session token found"))
            }
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

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // NOTE: \r, because there is a default prefix "Error: ", don't know how to remove it...
        // Not the best approach, there might be another solution
        writeln!(f, "{:>}{:>}", "[linked error]: ", self.to_string())
    }
}

pub fn default_error_handler(error: &Error, output: &mut dyn Write) {
    writeln!(output, "[error]: {}", error).ok();
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
