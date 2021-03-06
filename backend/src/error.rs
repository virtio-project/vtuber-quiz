use actix_http::body::BoxBody;
use actix_web::http::{header, StatusCode};
use actix_web::{error, HttpResponse};
use serde::Serialize;

use Error::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Sqlx(sqlx::Error),
    #[error(transparent)]
    Hcaptcha(#[from] crate::hcaptcha::HcaptchaError),
    #[error("database error")]
    Pg(#[from] crate::db::PgError),
    #[error("username has been taken")]
    ConflictUsername,
    #[error("unauthorized")]
    InvalidCredential,
    #[error("not found")]
    NotFound,
}

#[derive(Clone, Debug, Serialize)]
pub struct ErrorDisplay {
    pub code: u64,
    pub err: String,
}

impl Error {
    fn code(&self) -> u64 {
        match self {
            Sqlx(_) => 510000u64,
            Pg(_) => 511000u64,
            Hcaptcha(_) => 410000u64,
            InvalidCredential => 420000u64,
            ConflictUsername => 430001u64,
            NotFound => 440000u64,
        }
    }

    fn user_msg(&self) -> String {
        match self {
            Sqlx(_) => "database error".to_string(),
            Hcaptcha(e) => format!("{}", e),
            _ => format!("{}", self),
        }
    }

    fn as_display(&self) -> ErrorDisplay {
        ErrorDisplay {
            code: self.code(),
            err: self.user_msg(),
        }
    }
}

impl error::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Sqlx(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Pg(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Hcaptcha(_) => StatusCode::FORBIDDEN,
            InvalidCredential => StatusCode::FORBIDDEN,
            ConflictUsername => StatusCode::CONFLICT,
            NotFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        error!("{}", self);
        let body = serde_json::to_string(&self.as_display()).unwrap_or_else(|e| {
            error!("error occurred when generating error response: {}", e);
            r#"{"code":500000, "err":"internal server error"}"#.to_string()
        });
        let mut resp = HttpResponse::new(self.status_code()).set_body(BoxBody::new(body));
        resp.headers_mut().insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        resp
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        use sqlx::Error::*;
        match e {
            RowNotFound => Self::NotFound,
            _ => Self::Sqlx(e),
        }
    }
}
