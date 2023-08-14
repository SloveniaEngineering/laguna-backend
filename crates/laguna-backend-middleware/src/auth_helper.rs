use actix_web::{
  dev::{ServiceFactory, ServiceRequest},
  Error, Scope,
};
use jwt_compact::alg::Hs256Key;
use laguna_backend_model::role::Role;

use crate::auth::AuthorizationMiddlewareFactory;

// This is helper but we probably don't need this.
// This is also kinda stupid.
pub trait UseAuthorizationOnScope<T> {
  fn use_authorization(self, key: Hs256Key, min_role: Role, scope: Scope) -> Self;
}

impl<T> UseAuthorizationOnScope<T> for Scope<T>
where
  T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
{
  fn use_authorization(self, key: Hs256Key, min_role: Role, scope: Scope) -> Self {
    self.service(scope.wrap(AuthorizationMiddlewareFactory::new(key, min_role)))
  }
}
