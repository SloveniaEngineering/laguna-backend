use actix_web::get;
use actix_web::web;
use actix_web::HttpResponse;
use laguna_backend_model::misc::Laguna;

/// GET `/misc/laguna`
/// # Example
/// ### Request
/// ```bash
/// curl -X GET -i 'http://127.0.0.1:6969/misc/laguna'
/// ```
/// ### Response
/// ```json
/// {
///   "version": "0.1.0",
///   "authors": [
///     "kenpaicat <133065911+kenpaicat@users.noreply.github.com>",
///     "kozabrada123 <59031733+kozabrada123@users.noreply.github.com>",
///     "LinuxHeki <linuxheki@gmail.com>"],
///   "license": "Apache-2.0",
///   "description": "Laguna backend source tree",
///   "repository": "https://github.com/SloveniaEngineering/laguna-backend"
/// }
/// ```
#[get("/laguna")]
pub async fn get_app_info(laguna: web::Data<Laguna>) -> HttpResponse {
    HttpResponse::Ok().json(laguna)
}
