use actix_web::web;
use actix_web::HttpResponse;
use laguna_backend_dto::meta::AppInfoDTO;
use laguna_backend_middleware::mime::APPLICATION_LAGUNA_JSON_VERSIONED;

/// GET `/misc/laguna`
pub async fn get_app_info(laguna: web::Data<AppInfoDTO>) -> HttpResponse {
  HttpResponse::Ok()
    .content_type(APPLICATION_LAGUNA_JSON_VERSIONED)
    .json(laguna)
}

/// GET `/misc/healthcheck`
pub async fn healthcheck() -> HttpResponse {
  HttpResponse::Ok().finish()
}
