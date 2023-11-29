use actix_web::body::MessageBody;
use actix_web::dev::Transform;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse};

use actix_web::http::StatusCode;

use actix_web::{Error, HttpMessage, HttpResponse, ResponseError};

use std::fmt;

use futures_util::future::LocalBoxFuture;

use laguna_backend_dto::user::UserDTO;
use laguna_backend_model::role::Role;
use std::future::ready;
use std::future::Ready;

#[allow(missing_docs)]
pub struct AuthorizationMiddlewareFactory(Role);

impl AuthorizationMiddlewareFactory {
  pub fn new(min_role: Role) -> Self {
    Self(min_role)
  }
}

impl<S, B> Transform<S, ServiceRequest> for AuthorizationMiddlewareFactory
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
  B: 'static + MessageBody,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type InitError = ();
  type Transform = AuthorizationMiddleware<S>;
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ready(Ok(AuthorizationMiddleware {
      min_role: self.0,
      service,
    }))
  }
}

#[allow(missing_docs)]
pub struct AuthorizationMiddleware<S> {
  min_role: Role,
  service: S,
}

#[allow(missing_docs)]
#[derive(Debug)]
pub enum AuthorizationError {
  UnauthorizedRole { min_role: Role, actual_role: Role },
  NoToken,
}

impl fmt::Display for AuthorizationError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::UnauthorizedRole {
        min_role,
        actual_role,
      } => {
        write!(
          f,
          "Unauthorized role. Expected: {:?}, actual: {:?}",
          min_role, actual_role
        )
      },
      Self::NoToken => {
        write!(f, "No token")
      },
    }
  }
}

impl ResponseError for AuthorizationError {
  fn status_code(&self) -> StatusCode {
    match self {
      Self::UnauthorizedRole { .. } => StatusCode::UNAUTHORIZED,
      Self::NoToken => StatusCode::UNAUTHORIZED,
    }
  }

  fn error_response(&self) -> HttpResponse {
    match self {
      Self::UnauthorizedRole { .. } => HttpResponse::Unauthorized().body(format!("{}", self)),
      Self::NoToken => HttpResponse::Unauthorized().body(format!("{}", self)),
    }
  }
}

impl<S, B> Service<ServiceRequest> for AuthorizationMiddleware<S>
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
    let user_role = if let Some(user) = req.extensions().get::<UserDTO>() {
      user.role
    } else {
      return Box::pin(async move {
        Result::<Self::Response, Self::Error>::Err(AuthorizationError::NoToken.into())
      });
    };
    let min_role = self.min_role;
    if user_role < min_role {
      Box::pin(async move {
        Result::<Self::Response, Self::Error>::Err(
          AuthorizationError::UnauthorizedRole {
            min_role,
            actual_role: user_role,
          }
          .into(),
        )
      })
    } else {
      let fut = self.service.call(req);
      Box::pin(fut)
    }
  }
}
