use actix_web::body::{BoxBody, MessageBody};
use actix_web::dev::ServiceResponse;
use actix_web::dev::{forward_ready, Service, ServiceRequest, Transform};
use actix_web::error::Error;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use futures_util::future::LocalBoxFuture;
use jwt_compact::alg::{Hs256, Hs256Key};
use jwt_compact::{AlgorithmExt, UntrustedToken};
use laguna_backend_dto::user::UserDTO;
use std::fmt;
use std::future::ready;
use std::future::Ready;
use uuid::Uuid;

use crate::auth::AuthorizationError;
use crate::consts::ACCESS_TOKEN_HEADER_NAME;

pub struct ExclusiveMiddlewareFactory {
  key: Hs256Key,
}

impl ExclusiveMiddlewareFactory {
  pub fn new(key: Hs256Key) -> Self {
    Self { key }
  }
}

impl<S, B> Transform<S, ServiceRequest> for ExclusiveMiddlewareFactory
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
  B: 'static + MessageBody,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type InitError = ();
  type Transform = ExclusiveMiddleware<S>;
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ready(Ok(ExclusiveMiddleware {
      service,
      key: self.key.clone(),
    }))
  }
}

pub struct ExclusiveMiddleware<S> {
  key: Hs256Key,
  service: S,
}

#[derive(Debug)]
pub enum ExclusiveError {
  Exclusive,
}

impl fmt::Display for ExclusiveError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Exclusive => f.write_str("This endpoint is exclusive to the user."),
    }
  }
}

impl ResponseError for ExclusiveError {
  fn status_code(&self) -> StatusCode {
    match self {
      Self::Exclusive => StatusCode::FORBIDDEN,
    }
  }

  fn error_response(&self) -> HttpResponse<BoxBody> {
    HttpResponse::new(self.status_code()).set_body(BoxBody::new(self.to_string()))
  }
}

impl<S, B> Service<ServiceRequest> for ExclusiveMiddleware<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
  B: 'static + MessageBody,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  forward_ready!(service);

  fn call(&self, req: ServiceRequest) -> Self::Future {
    let access_token_header = req.headers().get(ACCESS_TOKEN_HEADER_NAME);
    if let Some(access_token_header) = access_token_header {
      // SECURITY: Token is trusted at this point but additional verification is however still performed.
      // NOTE: This is probably not a huge bottleneck and is a consequence of using external libraries for authentication (not authorization).
      let access_token = UntrustedToken::new(access_token_header.to_str().unwrap()).unwrap();
      let signed_access_token = Hs256
        .validate_for_signed_token::<UserDTO>(&access_token, &self.key)
        .map_err(AuthorizationError::Invalid);
      return match signed_access_token {
        Ok(signed_access_token) => {
          if signed_access_token.token.claims().custom.id
            != Uuid::parse_str(req.match_info().get("id").unwrap()).unwrap()
          {
            return Box::pin(async move {
              Result::<Self::Response, Self::Error>::Err(ExclusiveError::Exclusive.into())
            });
          }
          let fut = self.service.call(req);
          Box::pin(fut)
        },
        Err(err) => Box::pin(async move { Result::<Self::Response, Self::Error>::Err(err.into()) }),
      };
    }
    Box::pin(async move { Err(AuthorizationError::NoToken.into()) })
  }
}
