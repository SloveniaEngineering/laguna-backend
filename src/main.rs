use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use std::{env, io};

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let host = env::var("HOST").expect("HOST not specified");
    let port = env::var("PORT")
        .expect("PORT not specified")
        .parse::<u16>()
        .expect("PORT invalid");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .default_service(web::to(|| HttpResponse::NotFound()))
    })
    .bind((host, port))?
    .run()
    .await
}
