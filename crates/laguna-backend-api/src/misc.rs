use actix_web::get;
use actix_web::HttpResponse;
use laguna_backend_model::misc::Laguna;

#[get("/laguna")]
pub async fn get_app_info() -> HttpResponse {
    HttpResponse::Ok().json(Laguna {})
}
