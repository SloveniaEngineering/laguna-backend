use actix_web::web;
use actix_web::HttpResponse;
use laguna_backend_dto::meta::AppInfoDTO;

/// GET `/misc/laguna`
pub async fn get_app_info(laguna: web::Data<AppInfoDTO>) -> HttpResponse {
  HttpResponse::Ok().json(laguna)
}

/// GET `/misc/healthcheck`
pub async fn healthcheck() -> HttpResponse {
  HttpResponse::Ok().finish()
}
