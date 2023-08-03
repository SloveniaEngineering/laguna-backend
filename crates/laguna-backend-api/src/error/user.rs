use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::ResponseError;
use actix_web::{http::header::ContentType, HttpResponse};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum UserError {
    DoesNotExist,
    ExclusiveAccess,
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .content_type(ContentType::plaintext())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::DoesNotExist => StatusCode::BAD_REQUEST,
            Self::ExclusiveAccess => StatusCode::UNAUTHORIZED,
        }
    }
}
