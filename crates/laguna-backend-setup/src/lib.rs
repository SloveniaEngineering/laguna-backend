#![doc(html_logo_url = "https://sloveniaengineering.github.io/laguna-backend/logo.png")]
#![doc(html_favicon_url = "https://sloveniaengineering.github.io/laguna-backend/favicon.ico")]
#![doc(issue_tracker_base_url = "https://github.com/SloveniaEngineering/laguna-backend")]
use actix_cors::Cors;
use actix_jwt_auth_middleware::AuthenticationService;
use actix_jwt_auth_middleware::{Authority, TokenSigner};
use actix_settings::Mode;
use actix_web::dev::ServiceResponse;
use actix_web::dev::{ServiceFactory, ServiceRequest};
use actix_web::http::header;

use actix_web::middleware::DefaultHeaders;
use actix_web::web::ServiceConfig;

use actix_web::{web, App, HttpResponse};
use argon2::Argon2;
use argon2::{Algorithm, ParamsBuilder, Version};
use cached::proc_macro::once;
use chrono::Duration;
use jwt_compact::{alg::Hs256, alg::Hs256Key, TimeOptions};
use laguna_backend_api::login;
use laguna_backend_api::login::login;
use laguna_backend_api::meta;
use laguna_backend_api::meta::{get_app_info, healthcheck};
use laguna_backend_api::peer;
use laguna_backend_api::peer::peer_announce;
use laguna_backend_api::rating;
use laguna_backend_api::rating::{rating_create, rating_delete, rating_torrent_average};
use laguna_backend_api::register;
use laguna_backend_api::register::register;
use laguna_backend_api::torrent;
use laguna_backend_api::torrent::{
  torrent_delete, torrent_get, torrent_patch, torrent_put, torrent_swarm,
};
use laguna_backend_api::user;
use laguna_backend_api::user::{
  user_get, user_me_delete, user_me_get, user_patch, user_patch_me, user_role_change,
  user_torrents_get,
};
use laguna_backend_dto::already_exists::AlreadyExistsDTO;
use laguna_backend_dto::login::LoginDTO;
use laguna_backend_dto::meta::AppInfoDTO;
use laguna_backend_dto::peer::PeerDTO;
use laguna_backend_dto::rating::RatingDTO;
use laguna_backend_dto::register::RegisterDTO;
use laguna_backend_dto::role::RoleChangeDTO;
use laguna_backend_dto::torrent::{TorrentDTO, TorrentPatchDTO, TorrentPutDTO};
use laguna_backend_dto::torrent_rating::TorrentRatingDTO;
use laguna_backend_dto::user::{UserDTO, UserPatchDTO};

use laguna_backend_middleware::auth::AuthorizationMiddlewareFactory;
use laguna_backend_middleware::hexify::HexifyMiddlewareFactory;
use laguna_backend_middleware::mime::APPLICATION_LAGUNA_JSON_VERSIONED;
use laguna_backend_model::behaviour::Behaviour;
use laguna_backend_model::genre::Genre;
use laguna_backend_model::peer::Peer;
use laguna_backend_model::role::Role;
use laguna_backend_model::speedlevel::SpeedLevel;
use laguna_backend_model::torrent::Torrent;
use laguna_backend_model::torrent_rating::TorrentRating;
use laguna_backend_tracker_common::announce::AnnounceEvent;
use laguna_backend_tracker_common::info_hash::{InfoHash, SHA1_LENGTH};
use laguna_backend_tracker_common::peer::{PeerBin, PeerDict, PeerId, PeerStream};
use laguna_backend_tracker_http::announce::{Announce, AnnounceReply};
use laguna_config::make_overridable_with_env_vars;
use laguna_config::{Settings, LAGUNA_CONFIG};
use secrecy::ExposeSecret;
use sqlx::postgres::{PgPool, PgPoolOptions};

use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use std::sync::Once;

static ENV_LOGGER_INIT: Once = Once::new();

#[once(name = "SETTINGS")]
pub fn get_settings() -> Settings {
  Settings::parse_toml(LAGUNA_CONFIG).expect("Failed to parse settings")
}

// https://github.com/actix/actix-web/issues/2039
// https://github.com/actix/actix-web/issues/1190
pub fn setup() -> App<
  impl ServiceFactory<
    ServiceRequest,
    Config = (),
    Response = ServiceResponse,
    Error = actix_web::Error,
    InitError = (),
  >,
> {
  setup_with_settings(get_settings())
}

// https://github.com/actix/actix-web/blob/b1c85ba85be91b5ea34f31264853b411fadce1ef/actix-web/src/app.rs#L698
pub fn setup_with_settings(
  settings: Settings,
) -> App<
  impl ServiceFactory<
    ServiceRequest,
    Config = (),
    Response = ServiceResponse,
    Error = actix_web::Error,
    InitError = (),
  >,
> {
  App::new().configure(get_config_fn(settings))
}

pub fn get_config_fn(mut settings: Settings) -> impl FnOnce(&mut ServiceConfig) {
  make_overridable_with_env_vars(&mut settings);
  setup_logging(&settings);
  let secret_key = setup_secret_key(&settings);
  let (token_signer, authority) = crate::setup_authority!(secret_key, settings);
  let argon_context = setup_argon_context(&settings);

  move |service_config: &mut ServiceConfig| {
    service_config
      .app_data(web::Data::new(argon_context.clone()))
      // AuthenticationService by default doesnt include token_signer into app_data, hence we get it from setup_authority!() which is kinda hacky.
      .app_data(web::Data::new(token_signer.clone()))
      .app_data(web::Data::new(
        settings.application.tracker.announce_url.clone(),
      ))
      .service(
        web::scope("/api/user/auth")
          .route("/register", web::post().to(register))
          .route("/login", web::post().to(login)),
      )
      .service(
        web::scope("/misc")
          .route("/", web::get().to(get_app_info))
          .route("/healthcheck", web::get().to(healthcheck)),
      )
      .service(
        web::scope("peer")
          .wrap(HexifyMiddlewareFactory::new())
          .route("/announce", web::get().to(peer_announce::<SHA1_LENGTH>)),
      )
      // https://github.com/cloud-annotations/docusaurus-openapi/issues/231
      .service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
      )
      .service(Redoc::with_url("/redoc", ApiDoc::openapi()))
      .service(
        web::scope("/api")
          .wrap(
            DefaultHeaders::new().add((header::CONTENT_TYPE, APPLICATION_LAGUNA_JSON_VERSIONED)),
          )
          .wrap(AuthenticationService::new(authority))
          .service(
            web::scope("/user")
              .route("/me", web::patch().to(user_patch_me))
              .route(
                "/{id}",
                web::patch()
                  .wrap(AuthorizationMiddlewareFactory::new(
                    secret_key.clone(),
                    Role::Mod,
                  ))
                  .to(user_patch),
              )
              .route("/{id}/role_change", web::patch().to(user_role_change))
              .route("/me", web::get().to(user_me_get))
              .route("/{id}", web::get().to(user_get))
              .route("/me", web::delete().to(user_me_delete))
              .route("/{id}/torrents", web::get().to(user_torrents_get)),
          )
          .service(
            web::scope("/torrent")
              .route("/{info_hash}", web::get().to(torrent_get::<SHA1_LENGTH>))
              .route(
                "/",
                web::put()
                  .wrap(AuthorizationMiddlewareFactory::new(
                    secret_key.clone(),
                    Role::Verified,
                  ))
                  .to(torrent_put::<SHA1_LENGTH>),
              )
              .route("/rating", web::post().to(rating_create::<SHA1_LENGTH>))
              .route(
                "/rating/{info_hash}",
                web::delete().to(rating_delete::<SHA1_LENGTH>),
              )
              .route(
                "/rating/{info_hash}",
                web::get().to(rating_torrent_average::<SHA1_LENGTH>),
              )
              .route(
                "/{info_hash}",
                web::patch()
                  .wrap(AuthorizationMiddlewareFactory::new(
                    secret_key.clone(),
                    Role::Mod,
                  ))
                  .to(torrent_patch::<SHA1_LENGTH>),
              )
              .route(
                "/{info_hash}",
                web::delete()
                  .wrap(AuthorizationMiddlewareFactory::new(secret_key, Role::Mod))
                  .to(torrent_delete::<SHA1_LENGTH>),
              )
              .route(
                "/{info_hash}/swarm",
                web::get().to(torrent_swarm::<SHA1_LENGTH>),
              ),
          ),
      )
      .default_service(web::to(HttpResponse::NotFound));
  }
}

#[derive(OpenApi)]
#[openapi(
  info(description = "API documentation", title = "Laguna API"),
  components(
    schemas(
      UserDTO,
      UserPatchDTO,
      TorrentPutDTO,
      TorrentDTO,
      Torrent,
      Genre,
      TorrentPatchDTO,
      RatingDTO::<SHA1_LENGTH>,
      TorrentRatingDTO,
      TorrentRating,
      RegisterDTO,
      LoginDTO,
      AppInfoDTO,
      PeerDTO,
      AlreadyExistsDTO,
      Role,
      Behaviour,
      SpeedLevel,
      InfoHash<SHA1_LENGTH>,
      PeerId,
      Announce<SHA1_LENGTH>,
      AnnounceEvent,
      AnnounceReply,
      RoleChangeDTO,
      Peer,
      PeerStream,
      PeerDict,
      PeerBin,
    )
  ),
  paths(
    user::user_me_get,
    user::user_me_delete,
    user::user_patch_me,
    user::user_get,
    user::user_patch,
    user::user_torrents_get,
    torrent::torrent_get,
    torrent::torrent_put,
    torrent::torrent_patch,
    torrent::torrent_delete,
    torrent::torrent_swarm,
    rating::rating_create,
    rating::rating_delete,
    rating::rating_torrent_average,
    peer::peer_announce,
    register::register,
    login::login,
    meta::get_app_info,
    meta::healthcheck
  )
)]
struct ApiDoc;

#[inline]
pub fn get_loglevel(settings: &Settings) -> &str {
  match settings.actix.mode {
    Mode::Development => "debug",
    Mode::Production => "info",
  }
}

pub fn setup_logging(settings: &Settings) {
  if settings.actix.enable_log {
    ENV_LOGGER_INIT.call_once(|| {
      env_logger::init_from_env(env_logger::Env::new().default_filter_or(get_loglevel(settings)));
    });
  }
}

#[macro_export]
macro_rules! setup_authority {
  ($secret_key:ident, $settings:ident) => {{
    use ::laguna_backend_middleware::consts::ACCESS_TOKEN_HEADER_NAME;
    use ::laguna_backend_middleware::consts::REFRESH_TOKEN_HEADER_NAME;
    (
      TokenSigner::<UserDTO, Hs256>::new()
        .signing_key($secret_key.clone())
        .algorithm(Hs256)
        .access_token_name(ACCESS_TOKEN_HEADER_NAME)
        .refresh_token_name(REFRESH_TOKEN_HEADER_NAME)
        .access_token_lifetime(Duration::seconds(
          $settings.application.auth.access_token_lifetime_seconds,
        ))
        .refresh_token_lifetime(Duration::seconds(
          $settings.application.auth.refresh_token_lifetime_seconds,
        ))
        .build()
        .expect("Cannot create token signer"),
      Authority::<UserDTO, Hs256, _, _>::new()
        .refresh_authorizer(|| async move { Ok(()) })
        .enable_header_tokens(true)
        .enable_cookie_tokens(true)
        .time_options(TimeOptions::from_leeway(Duration::seconds(5)))
        .algorithm(Hs256)
        .access_token_name(ACCESS_TOKEN_HEADER_NAME)
        .refresh_token_name(REFRESH_TOKEN_HEADER_NAME)
        .verifying_key($secret_key.clone())
        .build()
        .expect("Cannot create key authority"),
    )
  }};
}

pub fn setup_argon_context(settings: &Settings) -> Argon2<'static> {
  let password_pepper = Box::leak::<'static>(
    settings
      .application
      .auth
      .password_pepper
      .expose_secret()
      .clone()
      .into_boxed_str(),
  );
  Argon2::new_with_secret(
    password_pepper.as_bytes(),
    Algorithm::Argon2id,
    Version::V0x13,
    ParamsBuilder::new()
      .p_cost(1)
      .m_cost(12288) // 12MiB in kibibytes
      .t_cost(3)
      .build()
      .unwrap(),
  )
  .unwrap()
}

pub fn setup_secret_key(settings: &Settings) -> Hs256Key {
  Hs256Key::new(
    settings
      .application
      .auth
      .secret_key
      .expose_secret()
      .as_str(),
  )
}

pub fn setup_cors(settings: &Settings) -> Cors {
  match settings.actix.mode {
    Mode::Development => Cors::permissive(),
    Mode::Production => Cors::default()
      .allowed_origin(settings.application.frontend.address().to_string().as_str())
      .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
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
      .max_age(3600),
  }
}

pub async fn setup_db(settings: &Settings) -> Result<PgPool, sqlx::Error> {
  let pool = PgPoolOptions::new()
    .max_connections(100)
    .connect(settings.application.database.url().as_str())
    .await?;
  // Run database migrations.
  // TODO: Can we not use ../../migrations but rather MIGRATIONS_DIR (requires ident macro resolve into literal).
  sqlx::migrate!("../../migrations").run(&pool).await?;

  Ok(pool)
}
