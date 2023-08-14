use actix_http::header::HeaderValue;
use actix_http::Request;

use actix_web::test::read_body_json;

use actix_web::web;
use actix_web::{
  dev::{Service, ServiceResponse},
  http::StatusCode,
  test::TestRequest,
};

use fake::{Fake, Faker};

use actix_web::test::init_service;
use laguna_backend_config::Settings;
use laguna_backend_dto::user::UserDTO;
use laguna_backend_dto::{login::LoginDTO, register::RegisterDTO};
use laguna_backend_setup::{setup, setup_with_settings};

use laguna_backend_middleware::consts::{ACCESS_TOKEN_HEADER_NAME, REFRESH_TOKEN_HEADER_NAME};

use sqlx::PgPool;

pub async fn setup_test(
  pool: &PgPool,
) -> impl Service<Request, Response = ServiceResponse, Error = actix_web::Error> {
  init_service(setup().app_data(web::Data::new(pool.clone()))).await
}

#[allow(dead_code)]
pub async fn setup_test_with_settings(
  settings: Settings,
  pool: &PgPool,
) -> impl Service<Request, Response = ServiceResponse, Error = actix_web::Error> {
  init_service(setup_with_settings(settings).app_data(web::Data::new(pool.clone()))).await
}

/// Registers and logs in a default user (Normie) with fake data.
pub async fn new_user(
  app: &impl Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
) -> (RegisterDTO, UserDTO, HeaderValue, HeaderValue) {
  new_user_with(Faker.fake::<RegisterDTO>(), &app).await
}

/// Registers and logs in a Verified user with fake data.
#[allow(dead_code)]
pub async fn new_verified_user(
  app: &impl Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
  pool: &PgPool,
) -> (RegisterDTO, UserDTO, HeaderValue, HeaderValue) {
  let (register_dto, user_dto, _, _) = new_user_with(Faker.fake::<RegisterDTO>(), &app).await;
  sqlx::query("UPDATE \"User\" SET role = 'Verified' WHERE id = $1")
    .bind(user_dto.id)
    .execute(pool)
    .await
    .expect("Unable to set user to 'Verified'");
  // Get the updated tokens for the updated user.
  let (user_dto, access_token, refresh_token) =
    login_user_safe(LoginDTO::from(register_dto.clone()), &app).await;
  (register_dto, user_dto, access_token, refresh_token)
}

/// Registers and logs in a Moderator user with fake data.
#[allow(dead_code)]
pub async fn new_mod_user(
  app: &impl Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
  pool: &PgPool,
) -> (RegisterDTO, UserDTO, HeaderValue, HeaderValue) {
  let (register_dto, user_dto, _, _) = new_user_with(Faker.fake::<RegisterDTO>(), &app).await;
  sqlx::query("UPDATE \"User\" SET role = 'Mod' WHERE id = $1")
    .bind(user_dto.id)
    .execute(pool)
    .await
    .expect("Unable to set user to 'Mod'");
  let (user_dto, access_token, refresh_token) =
    login_user_safe(LoginDTO::from(register_dto.clone()), &app).await;
  (register_dto, user_dto, access_token, refresh_token)
}

/// Registers and logs in an Admin user with fake data.
#[allow(dead_code)]
pub async fn new_admin_user(
  app: &impl Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
  pool: &PgPool,
) -> (RegisterDTO, UserDTO, HeaderValue, HeaderValue) {
  let (register_dto, user_dto, _, _) = new_user_with(Faker.fake::<RegisterDTO>(), &app).await;
  sqlx::query("UPDATE \"User\" SET role = 'Admin' WHERE id = $1")
    .bind(user_dto.id)
    .execute(pool)
    .await
    .expect("Unable to set user to 'Admin'");
  let (user_dto, access_token, refresh_token) =
    login_user_safe(LoginDTO::from(register_dto.clone()), &app).await;
  (register_dto, user_dto, access_token, refresh_token)
}

/// Registers and logs in a Normie user given a RegisterDTO.
pub async fn new_user_with(
  register_dto: RegisterDTO,
  app: &impl Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
) -> (RegisterDTO, UserDTO, HeaderValue, HeaderValue) {
  register_user_safe(register_dto.clone(), &app).await;
  let (user_dto, access_token, refresh_token) =
    login_user_safe(LoginDTO::from(register_dto.clone()), &app).await;
  (register_dto, user_dto, access_token, refresh_token)
}

pub async fn register_user_safe(
  register_dto: RegisterDTO,
  app: &impl Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
) {
  assert_eq!(
    register_user(register_dto, &app).await.status(),
    StatusCode::OK
  )
}

pub async fn login_user_safe(
  login_dto: LoginDTO,
  app: &impl Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
) -> (UserDTO, HeaderValue, HeaderValue) {
  let res = login_user(login_dto, &app).await;
  assert_eq!(res.status(), StatusCode::OK);
  let access_token = res
    .headers()
    .get(ACCESS_TOKEN_HEADER_NAME)
    .unwrap()
    .to_owned();
  let refresh_token = res
    .headers()
    .get(REFRESH_TOKEN_HEADER_NAME)
    .unwrap()
    .to_owned();
  let user_dto = read_body_json::<UserDTO, _>(res).await;
  (user_dto, access_token, refresh_token)
}

pub async fn register_user(
  register_dto: RegisterDTO,
  app: &impl Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
) -> ServiceResponse {
  app
    .call(
      TestRequest::post()
        .uri("/api/user/auth/register")
        .set_json(register_dto)
        .to_request(),
    )
    .await
    .unwrap()
}

pub async fn login_user(
  login_dto: LoginDTO,
  app: &impl Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
) -> ServiceResponse {
  app
    .call(
      TestRequest::post()
        .uri("/api/user/auth/login")
        .set_json(login_dto)
        .to_request(),
    )
    .await
    .unwrap()
}

#[allow(dead_code)]
pub async fn as_logged_in(
  access_token: HeaderValue,
  refresh_token: HeaderValue,
  mut req: TestRequest,
  app: &impl Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
) -> Result<ServiceResponse, actix_web::Error> {
  req = req
    .append_header((ACCESS_TOKEN_HEADER_NAME, access_token))
    .append_header((REFRESH_TOKEN_HEADER_NAME, refresh_token));
  app.call(req.to_request()).await
}

// Replaces the last character of the string with its successor, guaranteeing that the new string is different from the original.
// We need this because we have tests (using fake data) that require different strings than original which Faker cannot guarantee.
// TODO: Find a better way
#[allow(dead_code)]
pub fn different_string(string: String) -> String {
  string[..string.len() - 1].to_owned()
    + char::from_u32(string.chars().last().unwrap() as u32 + 1)
      .unwrap()
      .to_string()
      .as_str()
}
