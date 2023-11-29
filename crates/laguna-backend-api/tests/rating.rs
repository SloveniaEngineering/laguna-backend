use actix_web::test::read_body_json;
use actix_web::test::TestRequest;
use laguna_backend_dto::torrent_rating::TorrentRatingDTO;
use laguna_backend_dto::{rating::RatingDTO, torrent::TorrentDTO};
use laguna_backend_middleware::mime::APPLICATION_XBITTORRENT;
use laguna_backend_tracker_common::info_hash::SHA1_LENGTH;
use sqlx::PgPool;

mod common;

#[sqlx::test(migrations = "../../migrations")]
fn test_rating_create(pool: PgPool) -> sqlx::Result<(), sqlx::Error> {
  let app = common::setup_test(&pool).await;
  let (_, _, access_token, refresh_token) = common::new_verified_user(&app, &pool).await;
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

  assert_eq!(put_res.status(), 200);

  let torrent = read_body_json::<TorrentDTO<SHA1_LENGTH>, _>(put_res).await;

  let post_res = common::as_logged_in(
    access_token,
    refresh_token,
    TestRequest::post()
      .uri("/api/torrent/rating")
      .set_json(RatingDTO {
        info_hash: torrent.info_hash,
        rating: 10,
      }),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(post_res.status(), 200);
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
fn test_rating_create_twice(pool: PgPool) -> sqlx::Result<(), sqlx::Error> {
  let app = common::setup_test(&pool).await;
  let (_, _, access_token, refresh_token) = common::new_verified_user(&app, &pool).await;
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

  assert_eq!(put_res.status(), 200);

  let torrent = read_body_json::<TorrentDTO<SHA1_LENGTH>, _>(put_res).await;

  let post_res = common::as_logged_in(
    access_token.clone(),
    refresh_token.clone(),
    TestRequest::post()
      .uri("/api/torrent/rating")
      .set_json(RatingDTO {
        info_hash: torrent.info_hash.clone(),
        rating: 10,
      }),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(post_res.status(), 200);

  let post_res = common::as_logged_in(
    access_token,
    refresh_token,
    TestRequest::post()
      .uri("/api/torrent/rating")
      .set_json(RatingDTO {
        info_hash: torrent.info_hash,
        rating: 10,
      }),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(post_res.status(), 400);
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
fn test_rating_delete(pool: PgPool) -> sqlx::Result<(), sqlx::Error> {
  let app = common::setup_test(&pool).await;
  let (_, _, access_token, refresh_token) = common::new_verified_user(&app, &pool).await;
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

  assert_eq!(put_res.status(), 200);

  let torrent = read_body_json::<TorrentDTO<SHA1_LENGTH>, _>(put_res).await;

  let post_res = common::as_logged_in(
    access_token.clone(),
    refresh_token.clone(),
    TestRequest::post()
      .uri("/api/torrent/rating")
      .set_json(RatingDTO {
        info_hash: torrent.info_hash.clone(),
        rating: 10,
      }),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(post_res.status(), 200);

  let delete_res = common::as_logged_in(
    access_token,
    refresh_token,
    TestRequest::delete().uri(&format!("/api/torrent/rating/{}", torrent.info_hash)),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(delete_res.status(), 200);
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
fn test_rating_delete_twice(pool: PgPool) -> sqlx::Result<(), sqlx::Error> {
  let app = common::setup_test(&pool).await;
  let (_, _, access_token, refresh_token) = common::new_verified_user(&app, &pool).await;
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

  assert_eq!(put_res.status(), 200);

  let torrent = read_body_json::<TorrentDTO<SHA1_LENGTH>, _>(put_res).await;

  let post_res = common::as_logged_in(
    access_token.clone(),
    refresh_token.clone(),
    TestRequest::post()
      .uri("/api/torrent/rating")
      .set_json(RatingDTO {
        info_hash: torrent.info_hash.clone(),
        rating: 10,
      }),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(post_res.status(), 200);

  let delete_res = common::as_logged_in(
    access_token.clone(),
    refresh_token.clone(),
    TestRequest::delete().uri(&format!("/api/torrent/rating/{}", torrent.info_hash)),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(delete_res.status(), 200);

  let delete_res = common::as_logged_in(
    access_token,
    refresh_token,
    TestRequest::delete().uri(&format!("/api/torrent/rating/{}", torrent.info_hash)),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(delete_res.status(), 400);
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
fn test_rating_torrent_average(pool: PgPool) -> sqlx::Result<(), sqlx::Error> {
  let app = common::setup_test(&pool).await;
  let (_, _, access_token, refresh_token) = common::new_verified_user(&app, &pool).await;
  let (_, _, access_token_2, refresh_token_2) = common::new_user(&app).await;
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

  assert_eq!(put_res.status(), 200);
  let torrent = read_body_json::<TorrentDTO<SHA1_LENGTH>, _>(put_res).await;

  let post_res = common::as_logged_in(
    access_token.clone(),
    refresh_token.clone(),
    TestRequest::post()
      .uri("/api/torrent/rating")
      .set_json(RatingDTO {
        info_hash: torrent.info_hash.clone(),
        rating: 10,
      }),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(post_res.status(), 200);

  let post_res = common::as_logged_in(
    access_token_2.clone(),
    refresh_token_2.clone(),
    TestRequest::post()
      .uri("/api/torrent/rating")
      .set_json(RatingDTO {
        info_hash: torrent.info_hash.clone(),
        rating: 0,
      }),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(post_res.status(), 200);

  let torrent_rating_res = common::as_logged_in(
    access_token.clone(),
    refresh_token.clone(),
    TestRequest::get().uri(&format!("/api/torrent/rating/{}", torrent.info_hash)),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(torrent_rating_res.status(), 200);

  let torrent_rating = read_body_json::<TorrentRatingDTO, _>(torrent_rating_res).await;

  assert_eq!(torrent_rating.average.map(|f| f as i32), Some(5));
  assert_eq!(torrent_rating.count, Some(2));

  let delete_res = common::as_logged_in(
    access_token.clone(),
    refresh_token.clone(),
    TestRequest::delete().uri(&format!("/api/torrent/rating/{}", torrent.info_hash)),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(delete_res.status(), 200);

  let torrent_rating_res = common::as_logged_in(
    access_token_2,
    refresh_token_2,
    TestRequest::get().uri(&format!("/api/torrent/rating/{}", torrent.info_hash)),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(torrent_rating_res.status(), 200);

  let torrent_rating = read_body_json::<TorrentRatingDTO, _>(torrent_rating_res).await;

  assert_eq!(torrent_rating.average.map(|f| f as i32), Some(0));
  assert_eq!(torrent_rating.count, Some(1));

  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
fn test_rating_torrent_average_out_of_range(pool: PgPool) -> sqlx::Result<(), sqlx::Error> {
  let app = common::setup_test(&pool).await;
  let (_, _, access_token, refresh_token) = common::new_verified_user(&app, &pool).await;
  let (_, _, access_token_2, refresh_token_2) = common::new_user(&app).await;
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

  assert_eq!(put_res.status(), 200);

  let torrent = read_body_json::<TorrentDTO<SHA1_LENGTH>, _>(put_res).await;

  let post_res = common::as_logged_in(
    access_token_2.clone(),
    refresh_token_2.clone(),
    TestRequest::post()
      .uri("/api/torrent/rating")
      .set_json(RatingDTO {
        info_hash: torrent.info_hash.clone(),
        rating: 11,
      }),
    &app,
  )
  .await
  .unwrap();

  assert_eq!(post_res.status(), 400);
  Ok(())
}
