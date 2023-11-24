use actix_web::dev::PeerAddr;
use actix_web::web;
use actix_web::HttpResponse;
use laguna_backend_dto::meta::AppInfoDTO;
use laguna_backend_middleware::mime::APPLICATION_LAGUNA_JSON_VERSIONED;
use log::info;

#[allow(missing_docs)]
#[utoipa::path(
  get,
  path = "/api/misc/appinfo",
  responses(
    (status = 200, description = "Returns app info.", body = AppInfoDTO, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-alpha+json"),
  )
)]
pub async fn get_app_info(laguna: web::Data<AppInfoDTO>) -> HttpResponse {
  HttpResponse::Ok()
    .content_type(APPLICATION_LAGUNA_JSON_VERSIONED)
    .json(laguna)
}

#[allow(missing_docs)]
#[utoipa::path(
  get,
  path = "/api/misc/healthcheck",
  responses(
    (status = 200, description = "Returns healthcheck.", content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-alpha+json"),
  )
)]
pub async fn healthcheck(peer_addr: PeerAddr) -> HttpResponse {
  info!("Healthcheck from {}", peer_addr);
  HttpResponse::Ok().finish()
}
