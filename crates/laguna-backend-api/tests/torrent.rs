use std::str::FromStr;

use actix_http::{
  header::{self},
  StatusCode,
};

use actix_web::test::{read_body_json, TestRequest};

use chrono::{DateTime, Utc};

use laguna_backend_dto::torrent::{TorrentDTO, TorrentPatchDTO};
use laguna_backend_middleware::mime::APPLICATION_XBITTORRENT;
use laguna_backend_model::speedlevel::SpeedLevel;
use laguna_backend_tracker_common::info_hash::InfoHash;

use sqlx::PgPool;

mod common;

#[sqlx::test(migrations = "../../migrations")]
async fn test_get_torrent_bunny(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, user_dto, access_token, refresh_token) = common::new_user(&app).await;
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
  let expected_torrent_dto = TorrentDTO {
    info_hash: InfoHash::from(vec![
      175, 143, 16, 243, 11, 249, 174, 254, 207, 54, 134, 146, 43, 250, 13, 91, 210, 144, 163, 149,
    ]),
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
  assert_eq!(torrent_dto, expected_torrent_dto);

  let get_res = common::as_logged_in(
    access_token,
    refresh_token,
    TestRequest::get().uri(&format!("/api/torrent/{}", torrent_dto.info_hash)),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(get_res.status(), StatusCode::OK);
  let torrent_dto = read_body_json::<TorrentDTO, _>(get_res).await;
  assert_eq!(torrent_dto, expected_torrent_dto,);

  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_put_torrent(pool: PgPool) -> sqlx::Result<()> {
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
  let expected_torrent_dto = TorrentDTO {
    info_hash: InfoHash::from(vec![
      210, 71, 78, 134, 201, 91, 25, 184, 188, 253, 185, 43, 193, 44, 157, 68, 102, 124, 250, 54,
    ]),
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

  assert_eq!(torrent_dto, expected_torrent_dto);

  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_patch_torrent(pool: PgPool) -> sqlx::Result<()> {
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
  let expected_torrent_dto = TorrentDTO {
    info_hash: InfoHash::from(vec![
      210, 71, 78, 134, 201, 91, 25, 184, 188, 253, 185, 43, 193, 44, 157, 68, 102, 124, 250, 54,
    ]),
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

  assert_eq!(torrent_dto, expected_torrent_dto);

  let patch_res = common::as_logged_in(
    access_token,
    refresh_token,
    TestRequest::patch()
      .uri(&format!("/api/torrent/{}", torrent_dto.info_hash))
      .set_json(TorrentPatchDTO {
        title: String::from("New Title"),
        nfo: Some(String::from("New NFO")),
      }),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(patch_res.status(), StatusCode::OK);
  let torrent_dto = read_body_json::<TorrentDTO, _>(patch_res).await;

  let expected_torrent_dto = TorrentDTO {
    title: String::from("New Title"),
    nfo: Some(String::from("New NFO")),
    ..expected_torrent_dto
  };

  assert_eq!(torrent_dto, expected_torrent_dto);

  Ok(())
}
