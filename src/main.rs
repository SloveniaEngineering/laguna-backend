use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use std::env;

use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    // Logging level from RUST_LOG env variable.
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Database connection setup.
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL not set"))
        .await?;

    // Run database migrations.
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Server setup
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
    .bind((host, port))
    .expect("Cannot bind address")
    .run()
    .await
    .expect("Cannot start server");

    pool.close().await;

    Ok(())
}
