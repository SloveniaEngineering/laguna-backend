use actix_cors::Cors;

use actix_jwt_auth_middleware::use_jwt::UseJWTOnApp;

use actix_jwt_auth_middleware::Authority;
use actix_jwt_auth_middleware::TokenSigner;
use actix_web::http::header;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};

use chrono::Duration;
use jwt_compact::alg::Hs256;
use jwt_compact::alg::Hs256Key;
use jwt_compact::TimeOptions;
use laguna::api::login::login;
use laguna::api::misc::get_app_info;
use laguna::api::register::register;

use laguna::api::torrent::get_torrent;
use laguna::api::torrent::get_torrents_with_filter;
use laguna::api::torrent::put_torrent;
use laguna::api::user::delete_me;
use laguna::api::user::delete_user;
use laguna::api::user::get_me;
use laguna::api::user::get_user;
use laguna::middleware::consts::ACCESS_TOKEN_HEADER_NAME;
use laguna::middleware::consts::REFRESH_TOKEN_HEADER_NAME;
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
            .enable_header_tokens(true)
            .access_token_name(ACCESS_TOKEN_HEADER_NAME)
            .refresh_token_name(REFRESH_TOKEN_HEADER_NAME)
            .token_signer(Some(
                TokenSigner::new()
                    .signing_key(key.clone())
                    .algorithm(Hs256)
                    .time_options(TimeOptions::from_leeway(Duration::days(1)))
                    .build()
                    .expect("Cannot create token signer"),
            ))
            .verifying_key(key.clone())
            .build()
            .expect("Cannot create key authority");
        let cors = Cors::default()
            .allowed_origin(
                format!(
                    "{}:{}",
                    env::var("FRONTEND_HOST").expect("FRONTEND_HOST not set"),
                    env::var("FRONTEND_PORT").expect("FRONTEND_PORT not set")
                )
                .as_str(),
            )
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
            .allowed_headers(vec![
                header::ORIGIN,
                header::CONNECTION,
                header::ACCEPT,
                header::CONTENT_TYPE,
                header::REFERER,
                header::USER_AGENT,
                header::HOST,
                header::ACCEPT_ENCODING,
                header::ACCEPT_LANGUAGE,
                header::ACCESS_CONTROL_REQUEST_HEADERS,
            ])
            .max_age(3600);
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api/user/auth")
                    .service(register)
                    .service(login),
            )
            .use_jwt(
                authority,
                web::scope("/api")
                    .service(
                        web::scope("/user")
                            .service(get_me)
                            .service(get_user)
                            .service(
                                web::scope("/delete")
                                    .service(delete_me)
                                    .service(delete_user),
                            )
                            .service(web::scope("/misc").service(get_app_info)),
                    )
                    .service(
                        web::scope("/torrent")
                            .service(web::scope("/upload").service(put_torrent))
                            .service(get_torrents_with_filter)
                            .service(get_torrent),
                    ),
            )
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
