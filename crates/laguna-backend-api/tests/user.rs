use std::str::FromStr;

use actix_http::StatusCode;
use actix_web::{
  http::header,
  test::{read_body_json, TestRequest},
};

use chrono::{DateTime, Utc};
use laguna_backend_dto::{
  torrent::TorrentDTO,
  user::{UserDTO, UserPatchDTO},
};
use laguna_backend_middleware::mime::APPLICATION_XBITTORRENT;
use laguna_backend_model::speedlevel::SpeedLevel;
use sqlx::PgPool;

mod common;

#[sqlx::test(migrations = "../../migrations")]
async fn test_get_me(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, user_dto, access_token, refresh_token) = common::new_user(&app).await;
  let get_me_res = common::as_logged_in(
    access_token,
    refresh_token,
    TestRequest::with_uri("/api/user/me"),
    &app,
  )
  .await
  .unwrap();
  assert_eq!(get_me_res.status(), StatusCode::OK);
  assert_eq!(read_body_json::<UserDTO, _>(get_me_res).await, user_dto);
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_get_user(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, user_dto, access_token, refresh_token) = common::new_user(&app).await;
  let get_me_res = common::as_logged_in(
    access_token,
    refresh_token,
    TestRequest::with_uri(&format!("/api/user/{}", user_dto.id)),
    &app,
  )
  .await
  .unwrap();
  assert_eq!(get_me_res.status(), StatusCode::OK);
  assert_eq!(read_body_json::<UserDTO, _>(get_me_res).await, user_dto);
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_delete_me(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, _, access_token, refresh_token) = common::new_user(&app).await;
  let delete_me_res = common::as_logged_in(
    access_token,
    refresh_token,
    TestRequest::delete().uri("/api/user/me"),
    &app,
  )
  .await
  .unwrap();
  assert_eq!(delete_me_res.status(), StatusCode::OK);
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_patch_user(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, user_dto, access_token, refresh_token) = common::new_user(&app).await;
  let get_me_res = common::as_logged_in(
    access_token,
    refresh_token,
    TestRequest::patch()
      .uri(&format!("/api/user/{}", user_dto.id))
      .set_json(UserPatchDTO {
        avatar_url: Some(String::from("https://example.com")),
        is_history_private: false,
        is_profile_private: true,
      }),
    &app,
  )
  .await
  .unwrap();
  assert_eq!(get_me_res.status(), StatusCode::OK);
  let mut user_dto_expected = user_dto;
  user_dto_expected.avatar_url = Some(String::from("https://example.com"));
  user_dto_expected.is_history_private = false;
  user_dto_expected.is_profile_private = true;
  assert_eq!(
    read_body_json::<UserDTO, _>(get_me_res).await,
    user_dto_expected
  );
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_patch_user_remove_avatar_url(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, user_dto, access_token, refresh_token) = common::new_user(&app).await;
  let get_me_res = common::as_logged_in(
    access_token.clone(),
    refresh_token.clone(),
    TestRequest::patch()
      .uri(&format!("/api/user/{}", user_dto.id))
      .set_json(UserPatchDTO {
        avatar_url: Some(String::from("https://example.com")),
        is_history_private: false,
        is_profile_private: true,
      }),
    &app,
  )
  .await
  .unwrap();
  assert_eq!(get_me_res.status(), StatusCode::OK);
  let get_me_res = common::as_logged_in(
    access_token,
    refresh_token,
    TestRequest::patch()
      .uri(&format!("/api/user/{}", user_dto.id))
      .set_json(UserPatchDTO {
        avatar_url: None,
        is_history_private: false,
        is_profile_private: true,
      }),
    &app,
  )
  .await
  .unwrap();
  assert_eq!(get_me_res.status(), StatusCode::OK);
  let mut user_dto_expected = user_dto;
  user_dto_expected.avatar_url = None;
  user_dto_expected.is_history_private = false;
  user_dto_expected.is_profile_private = true;
  assert_eq!(
    read_body_json::<UserDTO, _>(get_me_res).await,
    user_dto_expected
  );
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_user_torrents_get(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, user_dto, access_token, refresh_token) = common::new_user(&app).await;
  let put_res = common::as_logged_in(
    access_token.clone(),
    refresh_token.clone(),
    TestRequest::put()
      .uri("/api/torrent/")
      .insert_header((header::CONTENT_TYPE.as_str(), APPLICATION_XBITTORRENT))
      .set_payload(include_bytes!("fixtures/webtorrent-fixtures/fixtures/leaves.torrent") as &[u8]),
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
    title: String::from("Leaves of Grass by Walt Whitman.epub"),
    file_name: String::from("Leaves of Grass by Walt Whitman.epub"),
    nfo: None,
    leech_count: 0,
    seed_count: 0,
    completed_count: 0,
    speedlevel: SpeedLevel::Lowspeed,
    creation_date: DateTime::<Utc>::from_str("2013-08-01T13:27:46Z").unwrap(),
    uploaded_at: torrent_dto.uploaded_at,
    uploaded_by: user_dto.id,
    modded_at: None,
    modded_by: None,
  };

  assert_eq!(torrent_dto, expected_torrent_dto_1);

  let put_res = common::as_logged_in(
    access_token.clone(),
    refresh_token.clone(),
    TestRequest::put()
      .uri("/api/torrent/")
      .insert_header((header::CONTENT_TYPE.as_str(), APPLICATION_XBITTORRENT))
      .set_payload(include_bytes!("fixtures/webtorrent-fixtures/fixtures/bunny.torrent") as &[u8]),
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
    title: String::from("bbb_sunflower_1080p_30fps_stereo_abl.mp4"),
    file_name: String::from("bbb_sunflower_1080p_30fps_stereo_abl.mp4"),
    nfo: None,
    leech_count: 0,
    seed_count: 0,
    completed_count: 0,
    speedlevel: SpeedLevel::Lowspeed,
    creation_date: DateTime::<Utc>::from_str("2013-12-17T19:48:21Z").unwrap(),
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
