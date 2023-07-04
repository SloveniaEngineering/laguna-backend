use actix_jwt_auth_middleware::use_jwt::UseJWTOnApp;

use actix_jwt_auth_middleware::Authority;
use actix_jwt_auth_middleware::TokenSigner;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};

use jwt_compact::alg::Hs256;
use jwt_compact::alg::Hs256Key;
use laguna::api::login::login;
use laguna::api::register::register;

use laguna::model::user::UserDTO;
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
    let key = Hs256Key::new("some random shit");
    let host = env::var("HOST").expect("HOST not specified");
    let port = env::var("PORT")
        .expect("PORT not specified")
        .parse::<u16>()
        .expect("PORT invalid");

    HttpServer::new(move || {
        let authority = Authority::<UserDTO, Hs256, _, _>::new()
            .refresh_authorizer(|| async move { Ok(()) })
            .token_signer(Some(
                TokenSigner::new()
                    .signing_key(key.clone())
                    .algorithm(Hs256)
                    .build()
                    .expect("Cannot create token signer"),
            ))
            .verifying_key(key.clone())
            .build()
            .expect("Cannot create key authority");
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(register)
            .service(login)
            .use_jwt(authority, web::scope("/api").service(web::scope("/user")))
            .default_service(web::to(|| HttpResponse::NotFound()))
    })
    .bind((host, port))
    .expect("Cannot bind address")
    .run()
    .await
    .expect("Cannot start server");

    // Is this necessary?
    // pool.close().await;

    Ok(())
}
