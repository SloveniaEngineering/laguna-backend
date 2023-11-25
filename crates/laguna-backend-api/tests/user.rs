use std::str::FromStr;

use actix_http::StatusCode;
use actix_web::test::{read_body_json, TestRequest, read_body};

use chrono::{DateTime, Utc};

use laguna_backend_api::error::user::UserError;
use laguna_backend_dto::{
  role::RoleChangeDTO,
  torrent::TorrentDTO,
  user::{UserDTO, UserPatchDTO},
};
use laguna_backend_middleware::mime::APPLICATION_XBITTORRENT;
use laguna_backend_model::{role::Role, speedlevel::SpeedLevel};
use sqlx::PgPool;

mod common;

#[sqlx::test(migrations = "../../migrations")]
async fn test_get_me(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, user_dto, access_token, refresh_token) = common::new_user(&app).await;
  let get_res = common::as_logged_in(
    access_token,
    refresh_token,
    TestRequest::with_uri("/api/user/me"),
    &app,
  )
  .await
  .unwrap();
  assert_eq!(get_res.status(), StatusCode::OK);
  assert_eq!(read_body_json::<UserDTO, _>(get_res).await, user_dto);
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_get_user(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, user_dto, access_token, refresh_token) = common::new_user(&app).await;
  let get_res = common::as_logged_in(
    access_token,
    refresh_token,
    TestRequest::with_uri(&format!("/api/user/{}", user_dto.id)),
    &app,
  )
  .await
  .unwrap();
  assert_eq!(get_res.status(), StatusCode::OK);
  assert_eq!(read_body_json::<UserDTO, _>(get_res).await, user_dto);
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_delete_me(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, _, access_token, refresh_token) = common::new_user(&app).await;
  let del_res = common::as_logged_in(
    access_token,
    refresh_token,
    TestRequest::delete().uri("/api/user/me"),
    &app,
  )
  .await
  .unwrap();
  assert_eq!(del_res.status(), StatusCode::OK);
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_user_patch(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, user_dto, access_token, refresh_token) = common::new_user(&app).await;
  let patch_res = common::as_logged_in(
    access_token.clone(),
    refresh_token.clone(),
    TestRequest::patch()
      .uri("/api/user/me")
      .set_json(UserPatchDTO {
        username: user_dto.username.clone(),
        avatar_url: Some(String::from("https://example.com")),
        is_profile_private: true,
      }),
    &app,
  )
  .await
  .unwrap();
  let new_access_token = patch_res
    .headers()
    .get("x-access-token")
    .unwrap()
    .to_owned();
  let new_refresh_token = patch_res
    .headers()
    .get("x-refresh-token")
    .unwrap()
    .to_owned();
  assert_eq!(patch_res.status(), StatusCode::OK);
  assert_eq!(
    read_body_json::<UserDTO, _>(patch_res).await,
    UserDTO {
      avatar_url: Some(String::from("https://example.com")),
      is_profile_private: true,
      ..user_dto
    }
  );
  assert_ne!(access_token, new_access_token);
  assert_ne!(refresh_token, new_refresh_token);
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_user_patch_change_username(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, user_dto, access_token, refresh_token) = common::new_user(&app).await;
  let patch_res = common::as_logged_in(
    access_token.clone(),
    refresh_token.clone(),
    TestRequest::patch()
      .uri("/api/user/me")
      .set_json(UserPatchDTO {
        username: String::from("new_username"),
        avatar_url: None,
        is_profile_private: false,
      }),
    &app,
  )
  .await
  .unwrap();
  assert_eq!(patch_res.status(), StatusCode::OK);
  assert_eq!(
    read_body_json::<UserDTO, _>(patch_res).await,
    UserDTO {
      username: String::from("new_username"),
      ..user_dto
    }
  );
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_user_patch_remove_avatar_url(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, user_dto, access_token, refresh_token) = common::new_user(&app).await;
  let patch_res = common::as_logged_in(
    access_token.clone(),
    refresh_token.clone(),
    TestRequest::patch()
      .uri("/api/user/me")
      .set_json(UserPatchDTO {
        username: user_dto.username.clone(),
        avatar_url: Some(String::from("https://example.com")),
        is_profile_private: true,
      }),
    &app,
  )
  .await
  .unwrap();
  assert_eq!(patch_res.status(), StatusCode::OK);
  let patch_res = common::as_logged_in(
    access_token,
    refresh_token,
    TestRequest::patch()
      .uri("/api/user/me")
      .set_json(UserPatchDTO {
        username: user_dto.username.clone(),
        avatar_url: None,
        is_profile_private: true,
      }),
    &app,
  )
  .await
  .unwrap();
  assert_eq!(patch_res.status(), StatusCode::OK);
  let mut user_dto_expected = user_dto;
  user_dto_expected.avatar_url = None;
  user_dto_expected.is_profile_private = true;
  assert_eq!(
    read_body_json::<UserDTO, _>(patch_res).await,
    user_dto_expected
  );
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_user_torrents_get(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, user_dto, access_token, refresh_token) = common::new_verified_user(&app, &pool).await;
  let put_res = common::as_logged_in(
    access_token.clone(),
    refresh_token.clone(),
    common::make_multipart(
      TestRequest::put().uri("/api/torrent/"),
      vec![common::MultipartField {
        name: b"torrent",
        filename: b"leaves.torrent",
        content_type: APPLICATION_XBITTORRENT,
        content: include_bytes!("fixtures/webtorrent-fixtures/fixtures/leaves.torrent"),
        boundary: b"aaabbbccc",
      }],
    ),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(put_res.status(), StatusCode::OK);
  let torrent_dto = read_body_json::<TorrentDTO, _>(put_res).await;
  let expected_torrent_dto_1 = TorrentDTO {
    info_hash: torrent_dto.info_hash.clone(),
    raw: include_bytes!("fixtures/webtorrent-fixtures/fixtures/leaves.torrent").to_vec(),
    announce_url: None,
    length: 362017,
    file_name: String::from("Leaves of Grass by Walt Whitman.epub"),
    nfo: None,
    genre: None,
    leech_count: 0,
    seed_count: 0,
    completed_count: 0,
    speedlevel: SpeedLevel::Lowspeed,
    is_freeleech: false,
    creation_date: DateTime::<Utc>::from_str("2013-08-01T13:27:46Z").unwrap(),
    created_by: Some(String::from("uTorrent/3300")),
    uploaded_at: torrent_dto.uploaded_at,
    uploaded_by: user_dto.id,
    modded_at: None,
    modded_by: None,
  };

  assert_eq!(torrent_dto, expected_torrent_dto_1);

  let put_res = common::as_logged_in(
    access_token.clone(),
    refresh_token.clone(),
    common::make_multipart(
      TestRequest::put().uri("/api/torrent/"),
      vec![common::MultipartField {
        name: b"torrent",
        filename: b"bunny.torrent",
        content_type: APPLICATION_XBITTORRENT,
        content: include_bytes!("fixtures/webtorrent-fixtures/fixtures/bunny.torrent"),
        boundary: b"aaabbbccc",
      }],
    ),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(put_res.status(), StatusCode::OK);
  let torrent_dto = read_body_json::<TorrentDTO, _>(put_res).await;
  let expected_torrent_dto_2 = TorrentDTO {
    info_hash: torrent_dto.info_hash.clone(),
    raw: include_bytes!("fixtures/webtorrent-fixtures/fixtures/bunny.torrent").to_vec(),
    announce_url: None,
    length: 434839491,
    file_name: String::from("bbb_sunflower_1080p_30fps_stereo_abl.mp4"),
    nfo: None,
    genre: None,
    leech_count: 0,
    seed_count: 0,
    completed_count: 0,
    speedlevel: SpeedLevel::Lowspeed,
    is_freeleech: false,
    creation_date: DateTime::<Utc>::from_str("2013-12-17T19:48:21Z").unwrap(),
    created_by: Some(String::from("uTorrent/3320")),
    uploaded_at: torrent_dto.uploaded_at,
    uploaded_by: user_dto.id,
    modded_at: None,
    modded_by: None,
  };
  assert_eq!(torrent_dto, expected_torrent_dto_2);

  let get_torrents_res = common::as_logged_in(
    access_token,
    refresh_token,
    TestRequest::get().uri(&format!("/api/user/{}/torrents", user_dto.id)),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(get_torrents_res.status(), StatusCode::OK);

  let torrents = read_body_json::<Vec<TorrentDTO>, _>(get_torrents_res).await;
  assert_eq!(torrents.len(), 2);

  assert_eq!(
    torrents,
    vec![expected_torrent_dto_1, expected_torrent_dto_2]
  );
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_user_role_change_self(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, user_dto, access_token, refresh_token) = common::new_user(&app).await;
  let role_change_res = common::as_logged_in(
    access_token.clone(),
    refresh_token.clone(),
    TestRequest::patch()
      .uri(&format!("/api/user/{}/role_change", user_dto.id))
      .set_json(RoleChangeDTO { to: Role::Verified }),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(
    role_change_res.status(),
    StatusCode::FORBIDDEN,
  );

  assert_eq!(
    read_body(role_change_res).await,
    UserError::SelfRoleChangeNotAllowed.to_string(),
  );
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_user_role_change_normie_to_verified_by_admin(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, _, admin_access_token, admin_refresh_token) = common::new_admin_user(&app, &pool).await;
  let (user_dto, normie_dto, normie_access_token, normie_refresh_token) =
    common::new_user(&app).await;
  let role_change_res = common::as_logged_in(
    admin_access_token.clone(),
    admin_refresh_token.clone(),
    TestRequest::patch()
      .uri(&format!("/api/user/{}/role_change", normie_dto.id))
      .set_json(RoleChangeDTO { to: Role::Verified }),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(role_change_res.status(), StatusCode::OK);

  // Changed user logs in and looks at their role
  let (verified_dto, verified_access_token, verified_refresh_token) =
    common::login_user_safe(user_dto.into(), &app).await;

  assert_eq!(verified_dto.role, Role::Verified);
  assert_ne!(verified_access_token, normie_access_token);
  assert_ne!(verified_refresh_token, normie_refresh_token);

  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_user_role_change_verified_to_normie_by_admin(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, _, admin_access_token, admin_refresh_token) = common::new_admin_user(&app, &pool).await;
  let (user_dto, verified_dto, verified_access_token, verified_refresh_token) =
    common::new_verified_user(&app, &pool).await;
  let role_change_res = common::as_logged_in(
    admin_access_token.clone(),
    admin_refresh_token.clone(),
    TestRequest::patch()
      .uri(&format!("/api/user/{}/role_change", verified_dto.id))
      .set_json(RoleChangeDTO { to: Role::Normie }),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(role_change_res.status(), StatusCode::OK);

  // Changed user logs in and looks at their role
  let (normie_dto, normie_access_token, normie_refresh_token) =
    common::login_user_safe(user_dto.into(), &app).await;

  assert_eq!(normie_dto.role, Role::Normie);
  assert_ne!(normie_access_token, verified_access_token);
  assert_ne!(normie_refresh_token, verified_refresh_token);

  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_user_role_change_normie_to_verified_by_mod(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, _, mod_access_token, mod_refresh_token) = common::new_mod_user(&app, &pool).await;
  let (user_dto, normie_dto, normie_access_token, normie_refresh_token) =
    common::new_user(&app).await;
  let role_change_res = common::as_logged_in(
    mod_access_token.clone(),
    mod_refresh_token.clone(),
    TestRequest::patch()
      .uri(&format!("/api/user/{}/role_change", normie_dto.id))
      .set_json(RoleChangeDTO { to: Role::Verified }),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(role_change_res.status(), StatusCode::OK);

  // Changed user logs in and looks at their role
  let (verified_dto, verified_access_token, verified_refresh_token) =
    common::login_user_safe(user_dto.into(), &app).await;

  assert_eq!(verified_dto.role, Role::Verified);
  assert_ne!(verified_access_token, normie_access_token);
  assert_ne!(verified_refresh_token, normie_refresh_token);

  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_user_role_change_verified_to_normie_by_mod(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, _, mod_access_token, mod_refresh_token) = common::new_mod_user(&app, &pool).await;
  let (user_dto, verified_dto, verified_access_token, verified_refresh_token) =
    common::new_verified_user(&app, &pool).await;
  let role_change_res = common::as_logged_in(
    mod_access_token.clone(),
    mod_refresh_token.clone(),
    TestRequest::patch()
      .uri(&format!("/api/user/{}/role_change", verified_dto.id))
      .set_json(RoleChangeDTO { to: Role::Normie }),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(role_change_res.status(), StatusCode::OK);

  // Changed user logs in and looks at their role
  let (normie_dto, normie_access_token, normie_refresh_token) =
    common::login_user_safe(user_dto.into(), &app).await;

  assert_eq!(normie_dto.role, Role::Normie);
  assert_ne!(normie_access_token, verified_access_token);
  assert_ne!(normie_refresh_token, verified_refresh_token);

  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_user_role_change_verified_to_mod_by_admin(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, _, admin_access_token, admin_refresh_token) = common::new_admin_user(&app, &pool).await;
  let (user_dto, verified_dto, verified_access_token, verified_refresh_token) =
    common::new_verified_user(&app, &pool).await;
  let role_change_res = common::as_logged_in(
    admin_access_token.clone(),
    admin_refresh_token.clone(),
    TestRequest::patch()
      .uri(&format!("/api/user/{}/role_change", verified_dto.id))
      .set_json(RoleChangeDTO { to: Role::Mod }),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(role_change_res.status(), StatusCode::OK);

  // Changed user logs in and looks at their role
  let (mod_dto, mod_access_token, mod_refresh_token) =
    common::login_user_safe(user_dto.into(), &app).await;

  assert_eq!(mod_dto.role, Role::Mod);
  assert_ne!(mod_access_token, verified_access_token);
  assert_ne!(mod_refresh_token, verified_refresh_token);

  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_user_role_change_mod_to_verified_by_admin(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, _, admin_access_token, admin_refresh_token) = common::new_admin_user(&app, &pool).await;
  let (user_dto, mod_dto, mod_access_token, mod_refresh_token) =
    common::new_mod_user(&app, &pool).await;
  let role_change_res = common::as_logged_in(
    admin_access_token.clone(),
    admin_refresh_token.clone(),
    TestRequest::patch()
      .uri(&format!("/api/user/{}/role_change", mod_dto.id))
      .set_json(RoleChangeDTO { to: Role::Verified }),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(role_change_res.status(), StatusCode::OK);

  // Changed user logs in and looks at their role
  let (verified_dto, verified_access_token, verified_refresh_token) =
    common::login_user_safe(user_dto.into(), &app).await;

  assert_eq!(verified_dto.role, Role::Verified);
  assert_ne!(verified_access_token, mod_access_token);
  assert_ne!(verified_refresh_token, mod_refresh_token);

  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_user_role_change_admin_to_mod_by_admin(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, _, admin_access_token, admin_refresh_token) = common::new_admin_user(&app, &pool).await;
  let (admin_register_dto, admin_dto, admin_2_access_token, admin_2_refresh_token) =
    common::new_admin_user(&app, &pool).await;
  let role_change_res = common::as_logged_in(
    admin_access_token.clone(),
    admin_refresh_token.clone(),
    TestRequest::patch()
      .uri(&format!("/api/user/{}/role_change", admin_dto.id))
      .set_json(RoleChangeDTO { to: Role::Mod }),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(role_change_res.status(), StatusCode::OK);

  // Changed user logs in and looks at their role
  let (mod_dto, mod_access_token, mod_refresh_token) =
    common::login_user_safe(admin_register_dto.into(), &app).await;

  assert_eq!(mod_dto.role, Role::Mod);
  assert_ne!(mod_access_token, admin_2_access_token);
  assert_ne!(mod_refresh_token, admin_2_refresh_token);

  Ok(())
}
