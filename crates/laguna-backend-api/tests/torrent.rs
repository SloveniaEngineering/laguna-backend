use std::str::FromStr;

use actix_http::StatusCode;

use actix_web::test::{read_body_json, TestRequest};

use chrono::{DateTime, Utc};

use laguna_backend_dto::{
  peer::PeerDTO,
  torrent::{TorrentDTO, TorrentPatchDTO},
};
use laguna_backend_middleware::mime::APPLICATION_XBITTORRENT;
use laguna_backend_model::{genre::Genre, speedlevel::SpeedLevel};

use sqlx::PgPool;

mod common;

#[sqlx::test(migrations = "../../migrations")]
async fn test_get_torrent_bunny(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, user_dto, access_token, refresh_token) = common::new_user(&app).await;
  let put_res = common::as_logged_in(
    access_token.clone(),
    refresh_token.clone(),
    common::make_multipart(
      TestRequest::put().uri("/api/torrent/"),
      vec![common::MultipartField {
        name: b"torrent",
        filename: b"bunny.torrent",
        content: include_bytes!("fixtures/webtorrent-fixtures/fixtures/bunny.torrent"),
        content_type: APPLICATION_XBITTORRENT,
        boundary: b"abbc761f78ff4d7cb7573b5a23f96ef0",
      }],
    ),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(put_res.status(), StatusCode::OK);
  let torrent_dto = read_body_json::<TorrentDTO, _>(put_res).await;
  let expected_torrent_dto = TorrentDTO {
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
    common::make_multipart(
      TestRequest::put().uri("/api/torrent/"),
      vec![common::MultipartField {
        name: b"torrent",
        filename: b"leaves.torrent",
        content: include_bytes!("fixtures/webtorrent-fixtures/fixtures/leaves.torrent"),
        content_type: APPLICATION_XBITTORRENT,
        boundary: b"abbc761f78ff4d7cb7573b5a23f96ef0",
      }],
    ),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(put_res.status(), StatusCode::OK);
  let torrent_dto = read_body_json::<TorrentDTO, _>(put_res).await;
  let expected_torrent_dto = TorrentDTO {
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
    common::make_multipart(
      TestRequest::put().uri("/api/torrent/"),
      vec![common::MultipartField {
        name: b"torrent",
        filename: b"leaves.torrent",
        content: include_bytes!("fixtures/webtorrent-fixtures/fixtures/leaves.torrent"),
        content_type: APPLICATION_XBITTORRENT,
        boundary: b"abbc761f78ff4d7cb7573b5a23f96ef0",
      }],
    ),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(put_res.status(), StatusCode::OK);
  let torrent_dto = read_body_json::<TorrentDTO, _>(put_res).await;
  let expected_torrent_dto = TorrentDTO {
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

  assert_eq!(torrent_dto, expected_torrent_dto);

  let patch_res = common::as_logged_in(
    access_token,
    refresh_token,
    TestRequest::patch()
      .uri(&format!("/api/torrent/{}", torrent_dto.info_hash))
      .set_json(TorrentPatchDTO {
        nfo: Some(String::from("New NFO")),
        genre: Some(Genre::Action),
      }),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(patch_res.status(), StatusCode::OK);
  let torrent_dto = read_body_json::<TorrentDTO, _>(patch_res).await;
  let expected_torrent_dto = TorrentDTO {
    nfo: Some(String::from("New NFO")),
    genre: Some(Genre::Action),
    ..expected_torrent_dto
  };

  assert_eq!(torrent_dto, expected_torrent_dto);

  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_delete_torrent(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, _, access_token, refresh_token) = common::new_user(&app).await;
  let put_res = common::as_logged_in(
    access_token.clone(),
    refresh_token.clone(),
    common::make_multipart(
      TestRequest::put().uri("/api/torrent/"),
      vec![common::MultipartField {
        name: b"torrent",
        filename: b"leaves.torrent",
        content: include_bytes!("fixtures/webtorrent-fixtures/fixtures/leaves.torrent"),
        content_type: APPLICATION_XBITTORRENT,
        boundary: b"abbc761f78ff4d7cb7573b5a23f96ef0",
      }],
    ),
    &app,
  )
  .await
  .unwrap();
  assert_eq!(put_res.status(), StatusCode::OK);
  let torrent_dto = read_body_json::<TorrentDTO, _>(put_res).await;
  let delete_res = common::as_logged_in(
    access_token,
    refresh_token,
    TestRequest::delete().uri(&format!("/api/torrent/{}", torrent_dto.info_hash)),
    &app,
  )
  .await
  .unwrap();
  assert_eq!(delete_res.status(), StatusCode::OK);
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_get_torrent_swarm(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, _, access_token, refresh_token) = common::new_user(&app).await;
  let put_res = common::as_logged_in(
    access_token.clone(),
    refresh_token.clone(),
    common::make_multipart(
      TestRequest::put().uri("/api/torrent/"),
      vec![common::MultipartField {
        name: b"torrent",
        filename: b"bunny.torrent",
        content: include_bytes!("fixtures/webtorrent-fixtures/fixtures/bunny.torrent"),
        content_type: APPLICATION_XBITTORRENT,
        boundary: b"abbc761f78ff4d7cb7573b5a23f96ef0",
      }],
    ),
    &app,
  )
  .await
  .unwrap();
  assert_eq!(put_res.status(), StatusCode::OK);

  let torrent_dto = read_body_json::<TorrentDTO, _>(put_res).await;

  let get_res = common::as_logged_in(
    access_token,
    refresh_token,
    TestRequest::get().uri(&format!("/api/torrent/{}/swarm", torrent_dto.info_hash)),
    &app,
  )
  .await
  .unwrap();
  assert_eq!(get_res.status(), StatusCode::OK);
  assert_eq!(read_body_json::<Vec<PeerDTO>, _>(get_res).await, vec![]);

  Ok(())
}
