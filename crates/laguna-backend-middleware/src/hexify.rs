use std::future::{ready, Ready};

use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::uri::PathAndQuery;
use actix_web::http::Uri;
use actix_web::web::Bytes;
use actix_web::Error;
use percent_encoding::percent_decode_str;
use qstring::QString;

#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct HexifyMiddlewareFactory;

impl HexifyMiddlewareFactory {
  /// New hexify middleware instance.
  pub fn new() -> Self {
    Self
  }
}

impl Default for HexifyMiddlewareFactory {
  fn default() -> Self {
    Self::new()
  }
}

impl<S, B> Transform<S, ServiceRequest> for HexifyMiddlewareFactory
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Transform = HexifyMiddleware<S>;
  type InitError = ();
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ready(Ok(HexifyMiddleware { service }))
  }
}

#[allow(missing_docs)]
pub struct HexifyMiddleware<S> {
  service: S,
}

impl<S, B> Service<ServiceRequest> for HexifyMiddleware<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = S::Future;

  forward_ready!(service);

  fn call(&self, mut req: ServiceRequest) -> Self::Future {
    // TODO: Get rid of QString and use actix's methods
    let s = QString::from(req.query_string());

    if let Some(info_hash) = s.get("info_hash") {
      if info_hash.contains('%') {
        let hex_info_hash = percent_decode_str(info_hash)
          .map(|b| format!("{:02x}", b))
          .collect::<String>();
        let mut parts = req.head().uri.clone().into_parts();
        let query = parts.path_and_query.as_ref().and_then(|pq| pq.query());
        let path = match query {
          Some(q) => Bytes::from(format!(
            "{}?{}",
            req.path(),
            q.replace(info_hash, &hex_info_hash)
          )),
          None => Bytes::from(req.path().to_string()),
        };
        parts.path_and_query = Some(PathAndQuery::from_maybe_shared(path).unwrap());
        let uri = Uri::from_parts(parts).unwrap();
        req.match_info_mut().get_mut().update(&uri);
        req.head_mut().uri = uri;
      }
    }

    if let Some(peer_id) = s.get("peer_id") {
      let hex_peer_id = percent_decode_str(peer_id)
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
      let mut parts = req.head().uri.clone().into_parts();
      let query = parts.path_and_query.as_ref().and_then(|pq| pq.query());
      let path = match query {
        Some(q) => Bytes::from(format!(
          "{}?{}",
          req.path(),
          q.replace(peer_id, &hex_peer_id)
        )),
        None => Bytes::from(req.path().to_string()),
      };
      parts.path_and_query = Some(PathAndQuery::from_maybe_shared(path).unwrap());
      let uri = Uri::from_parts(parts).unwrap();
      req.match_info_mut().get_mut().update(&uri);
      req.head_mut().uri = uri;
    }

    self.service.call(req)
  }
}
