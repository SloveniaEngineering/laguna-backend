use std::str::FromStr;

use actix_http::StatusCode;

use actix_web::test::{read_body, read_body_json, TestRequest};

use bendy::decoding::FromBencode;
use chrono::{DateTime, Utc};

use digest::Digest;
use laguna_backend_dto::{
  peer::PeerDTO,
  torrent::{TorrentDTO, TorrentFile, TorrentPatchDTO},
};
use laguna_backend_middleware::mime::APPLICATION_XBITTORRENT;
use laguna_backend_model::{
  download::{Download, DownloadHash},
  genre::Genre,
  speedlevel::SpeedLevel,
};

use laguna_backend_tracker_common::info_hash::SHA1_LENGTH;
use sha2::Sha256;
use sqlx::PgPool;

mod common;

#[sqlx::test(migrations = "../../migrations")]
async fn test_get_torrent_bunny(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, user_dto, access_token, refresh_token) = common::new_verified_user(&app, &pool).await;
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
async fn test_get_torrent_bunny_raw(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, user_dto, access_token, refresh_token) = common::new_verified_user(&app, &pool).await;
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
    TestRequest::get().uri(&format!("/api/torrent/{}/raw", torrent_dto.info_hash)),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(get_res.status(), StatusCode::OK);

  let torrent_bytes = &read_body(get_res).await[..];

  let torrent = TorrentFile::from_bencode(torrent_bytes).unwrap();

  // check if Download was inserted
  // aka.   queries/download_lookup.sql
  let download = sqlx::query_as::<_, Download<SHA1_LENGTH>>(
    "SELECT * FROM \"Download\" WHERE info_hash = $1 AND user_id = $2",
  )
  .bind(torrent_dto.info_hash.clone())
  .bind(user_dto.id)
  .fetch_one(&pool)
  .await?;

  assert_eq!(
    torrent.announce_url,
    Some(format!(
      "http://127.0.0.1:6969/peer/announce?down_hash={}",
      download.down_hash
    ))
  );

  assert_eq!(download.info_hash, torrent_dto.info_hash);
  assert_eq!(download.user_id, user_dto.id);
  assert_eq!(
    download.down_hash,
    DownloadHash::from(
      Sha256::digest(
        [
          &torrent_dto.info_hash.0,
          &user_dto.id.to_bytes_le()[..],
          &download.ts.timestamp().to_le_bytes()
        ]
        .concat()
      )
      .to_vec()
    )
  );

  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_put_torrent(pool: PgPool) -> sqlx::Result<()> {
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
async fn test_put_torrent_unverified(pool: PgPool) -> sqlx::Result<()> {
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
  .await;

  assert!(put_res.is_err());

  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_patch_torrent(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, user_dto, access_token, refresh_token) = common::new_mod_user(&app, &pool).await;
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
  let (_, _, access_token, refresh_token) = common::new_mod_user(&app, &pool).await;
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
  let (_, _, access_token, refresh_token) = common::new_verified_user(&app, &pool).await;
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
