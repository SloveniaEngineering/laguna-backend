use actix_web::body::MessageBody;
use actix_web::dev::Transform;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse};
use core::fmt::Debug;
use std::sync::Arc;

use actix_web::http::StatusCode;

use actix_web::{Error, FromRequest, HttpResponse, ResponseError};
use std::fmt;
use std::marker::PhantomData;
use uuid::Uuid;

use futures_util::future::LocalBoxFuture;
use jwt_compact::alg::{Hs256, Hs256Key};
use jwt_compact::{AlgorithmExt, UntrustedToken};
use laguna_backend_dto::user::UserDTO;

use std::future::ready;
use std::future::Ready;

use crate::consts::ACCESS_TOKEN_HEADER_NAME;

pub trait Identity {
    fn id(&self) -> Uuid;
}

pub struct ExclusiveMiddlewareFactory<ReqFormat> {
    key: Hs256Key,
    request_format_marker: PhantomData<ReqFormat>,
}

impl<ReqFormat> ExclusiveMiddlewareFactory<ReqFormat> {
    pub fn new(key: Hs256Key) -> Self {
        Self {
            key,
            request_format_marker: PhantomData,
        }
    }
}

impl<S, B, ReqFormat> Transform<S, ServiceRequest> for ExclusiveMiddlewareFactory<ReqFormat>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static + MessageBody,
    ReqFormat: FromRequest + Identity + Debug,
    ReqFormat::Error: Debug,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ExclusiveMiddleware<S, ReqFormat>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ExclusiveMiddleware {
            key: self.key.clone(),
            service,
            request_format_marker: PhantomData,
        }))
    }
}

pub struct ExclusiveMiddleware<S, ReqFormat> {
    key: Hs256Key,
    service: S,
    request_format_marker: PhantomData<ReqFormat>,
}

#[derive(Debug)]
pub enum ExclusiveError {
    ExclusiveEndpoint,
}

impl fmt::Display for ExclusiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExclusiveEndpoint => write!(f, "Endpoint is exclusive to specific user."),
        }
    }
}

impl ResponseError for ExclusiveError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::ExclusiveEndpoint => StatusCode::BAD_REQUEST,
        }
    }
}

impl<S, B, ReqFormat> Service<ServiceRequest> for ExclusiveMiddleware<S, ReqFormat>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static + MessageBody,
    ReqFormat: FromRequest + Identity + Debug,
    ReqFormat::Error: Debug,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let access_token_header = req.headers().get(ACCESS_TOKEN_HEADER_NAME);
        if let Some(access_token_header) = access_token_header {
            let access_token = UntrustedToken::new(access_token_header.to_str().unwrap()).unwrap();
            let _integrity = Hs256
                .validate_integrity::<UserDTO>(&access_token, &self.key)
                .unwrap();
            let _req_arc = Arc::new(req);
            /*
            let fut = self.service.call(req);
            return Box::pin(async move {
                let (req, payload) = req.into_parts();
                let dto = ReqFormat::from_request(&req, &mut payload).await.unwrap();
                if dto.id() != integrity.claims().custom.id {
                    Result::<Self::Response, Self::Error>::Err(ExclusiveError::ExclusiveEndpoint.into())
                } else {
                    self.service.call(ServiceRequest::from_parts(req, payload)).await
                }
                self.service.call(ServiceRequest::from_parts(req, payload)).await
            })
            */
            todo!()
        }

        Box::pin(async move { Err(ExclusiveError::ExclusiveEndpoint.into()) })
    }
}
