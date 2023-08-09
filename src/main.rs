#![doc(html_logo_url = "https://sloveniaengineering.github.io/laguna-backend/logo.png")]
#![doc(html_favicon_url = "https://sloveniaengineering.github.io/laguna-backend/favicon.ico")]
#![doc(issue_tracker_base_url = "https://github.com/SloveniaEngineering/laguna-backend")]
use actix_cors::Cors;

use actix_jwt_auth_middleware::use_jwt::UseJWTOnApp;

use actix_jwt_auth_middleware::Authority;
use actix_jwt_auth_middleware::TokenSigner;
use actix_web::http::header;

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};

use chrono::Duration;
use jwt_compact::alg::Hs256;
use jwt_compact::alg::Hs256Key;

use laguna::api::login::login;
use laguna::api::misc::get_app_info;
use laguna::api::register::register;

use jwt_compact::TimeOptions;
use laguna::api::torrent::torrent_get;
use laguna::api::torrent::torrent_patch;
use laguna::api::torrent::torrent_put;
use laguna::api::user::user_delete;
use laguna::api::user::user_get;
use laguna::api::user::user_me_delete;
use laguna::api::user::user_me_get;

use laguna::api::user::user_patch;
use laguna::api::user::user_peers_get;
use laguna::dto::meta::AppInfoDTO;
use laguna::middleware::auth::AuthorizationMiddlewareFactory;
use laguna::middleware::consts::ACCESS_TOKEN_HEADER_NAME;
use laguna::middleware::consts::REFRESH_TOKEN_HEADER_NAME;

use laguna::dto::user::UserDTO;
use laguna::model::role::Role;
use laguna_config::make_overridable_with_env_vars;
use laguna_config::CONFIG_DEV;

use std::env;

use actix_settings::ApplySettings;
use laguna_config::Settings;
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    let mut settings = Settings::parse_toml(CONFIG_DEV).expect("Failed to parse settings");

    make_overridable_with_env_vars(&mut settings);

    if settings.actix.enable_log {
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    }

    // Database connection setup.
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(settings.application.database.url().as_str())
        .await?;

    // Run database migrations.
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Server setup
    let secret_key = Hs256Key::new(
        settings
            .application
            .auth
            .secret_key
            .expose_secret()
            .as_str(),
    );
    let frontend_address = settings.application.frontend.address();
    let pool_clone = pool.clone();

    HttpServer::new(move || {
        let authority = Authority::<UserDTO, Hs256, _, _>::new()
            .refresh_authorizer(|| async move { Ok(()) })
            .enable_header_tokens(true)
            .access_token_name(ACCESS_TOKEN_HEADER_NAME)
            .refresh_token_name(REFRESH_TOKEN_HEADER_NAME)
            .time_options(TimeOptions::from_leeway(Duration::seconds(5)))
            .token_signer(Some(
                TokenSigner::new()
                    .signing_key(secret_key.clone())
                    .algorithm(Hs256)
                    .access_token_name(ACCESS_TOKEN_HEADER_NAME)
                    .refresh_token_name(REFRESH_TOKEN_HEADER_NAME)
                    .access_token_lifetime(Duration::seconds(
                        settings.application.auth.access_token_lifetime_seconds,
                    ))
                    .refresh_token_lifetime(Duration::seconds(
                        settings.application.auth.refresh_token_lifetime_seconds,
                    ))
                    .build()
                    .expect("Cannot create token signer"),
            ))
            .verifying_key(secret_key.clone())
            .build()
            .expect("Cannot create key authority");
        let cors = Cors::default()
            .allowed_origin(frontend_address.to_string().as_str())
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
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(AppInfoDTO {
                version: env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION not set"),
                authors: env::var("CARGO_PKG_AUTHORS")
                    .expect("CARGO_PKG_AUTHORS not set")
                    .split(":")
                    .map(ToString::to_string)
                    .collect::<Vec<String>>(),
                license: env::var("CARGO_PKG_LICENSE").expect("CARGO_PKG_LICENSE not set"),
                description: env::var("CARGO_PKG_DESCRIPTION")
                    .expect("CARGO_PKG_DESCRIPTION not set"),
                repository: env::var("CARGO_PKG_REPOSITORY").expect("CARGO_PKG_REPOSITORY not set"),
            }))
            .service(
                web::scope("/api/user/auth")
                    .route("/register", web::post().to(register))
                    .route("/login", web::post().to(login)),
            )
            .service(web::scope("/misc").route("/", web::get().to(get_app_info)))
            .use_jwt(
                authority,
                web::scope("/api")
                    .service(
                        web::scope("/user")
                            .route("/", web::patch().to(user_patch))
                            .route("/me", web::get().to(user_me_get))
                            .route("/{id}", web::get().to(user_get))
                            .route("/me", web::delete().to(user_me_delete))
                            .route(
                                "/{id}",
                                web::delete().to(user_delete).wrap(
                                    AuthorizationMiddlewareFactory::new(
                                        secret_key.clone(),
                                        Role::Admin,
                                    ),
                                ),
                            )
                            .route("/{id}/peers", web::get().to(user_peers_get)),
                    )
                    .service(
                        web::scope("/torrent")
                            .route("/", web::get().to(torrent_get))
                            .route("/", web::put().to(torrent_put))
                            .route("/", web::patch().to(torrent_patch)),
                    ),
            )
            .default_service(web::to(|| HttpResponse::NotFound()))
    })
    .apply_settings(&settings)
    .run()
    .await
    .expect("Cannot start server");

    pool_clone.close().await;

    Ok(())
}
