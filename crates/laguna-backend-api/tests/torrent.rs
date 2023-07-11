use actix_http::StatusCode;
use actix_web::test::{read_body_json, TestRequest};
use laguna_backend_api::state::{TorrentState, UserState};
use laguna_backend_middleware::filters::torrent::TorrentFilter;
use laguna_backend_model::{
    login::LoginDTO,
    register::RegisterDTO,
    torrent::{Torrent, TorrentPutDTO, TorrentDTO},
    user::User,
};

mod common;

#[actix_web::test]
async fn test_torrent_upload() {
    let (pool, app) = common::setup().await;
    let login_res = common::register_and_login_new_user(
        RegisterDTO {
            username: String::from("test_torrent_upload"),
            email: String::from("test_torrent_upload"),
            password: String::from("test123"),
        },
        LoginDTO {
            username_or_email: String::from("test_torrent_upload"),
            password: String::from("test123"),
        },
        &app,
    )
    .await;
    assert_eq!(login_res.status(), StatusCode::OK);

    let user = sqlx::query_as::<_, User>("SELECT * FROM \"User\" WHERE username = $1")
        .bind("test_torrent_upload")
        .fetch_one(&pool)
        .await
        .unwrap();

    let res = common::request_with_jwt(
        &login_res,
        TestRequest::put()
            .uri("/api/torrent/upload/")
            .set_json(TorrentPutDTO {
                title: String::from("TEST TORRENT UPLOAD"),
                file_name: String::from("test_upload"),
                nfo: None,
                uploaded_by: user.id,
                modded_by: None,
                payload: b"some random torrent bytes".to_vec(),
            }),
        &app,
    )
    .await;

    assert_eq!(
        read_body_json::<UserState, _>(login_res).await,
        UserState::LoginSuccess { user: user.into() }
    );

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        read_body_json::<TorrentState, _>(res).await,
        TorrentState::UploadSuccess
    );

    common::teardown(pool).await;
}

#[actix_web::test]
#[ignore = "Not yet fully implemented"]
async fn test_torrent_download() {
    let (pool, app) = common::setup().await;
    let login_res = common::register_and_login_new_user(
        RegisterDTO {
            username: String::from("test_torrent_download"),
            email: String::from("test_torrent_download"),
            password: String::from("test123"),
        },
        LoginDTO {
            username_or_email: String::from("test_torrent_download"),
            password: String::from("test123"),
        },
        &app,
    )
    .await;
    assert_eq!(login_res.status(), StatusCode::OK);

    let user = sqlx::query_as::<_, User>("SELECT * FROM \"User\" WHERE username = $1")
        .bind("test_torrent_download")
        .fetch_one(&pool)
        .await
        .unwrap();

    let res = common::request_with_jwt(
        &login_res,
        TestRequest::put()
            .uri("/api/torrent/upload/")
            .set_json(TorrentPutDTO {
                title: String::from("TEST TORRENT DOWNLOAD"),
                file_name: String::from("test_download"),
                nfo: None,
                uploaded_by: user.id,
                modded_by: None,
                payload: b"some random torrent bytes".to_vec(),
            }),
        &app,
    )
    .await;

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        read_body_json::<TorrentState, _>(res).await,
        TorrentState::UploadSuccess
    );

    let torrent = sqlx::query_as::<_, Torrent>("SELECT * FROM \"Torrent\" WHERE title = $1")
        .bind("TEST TORRENT DOWNLOAD")
        .fetch_one(&pool)
        .await
        .unwrap();

    let res = common::request_with_jwt(
        &login_res,
        TestRequest::get().uri(&format!("/api/torrent/download/{}", torrent.id)),
        &app,
    )
    .await;

    assert_eq!(res.status(), StatusCode::OK);

    assert_eq!(
        read_body_json::<UserState, _>(login_res).await,
        UserState::LoginSuccess { user: user.into() }
    );
}

#[actix_web::test]
async fn test_torrent_get() {
    let (pool, app) = common::setup().await;
    let login_res = common::register_and_login_new_user(
        RegisterDTO {
            username: String::from("test_torrent_get"),
            email: String::from("test_torrent_get"),
            password: String::from("test123"),
        },
        LoginDTO {
            username_or_email: String::from("test_torrent_get"),
            password: String::from("test123"),
        },
        &app,
    )
    .await;
    assert_eq!(login_res.status(), StatusCode::OK);

    let user = sqlx::query_as::<_, User>("SELECT * FROM \"User\" WHERE username = $1")
        .bind("test_torrent_get")
        .fetch_one(&pool)
        .await
        .unwrap();

    let res = common::request_with_jwt(
        &login_res,
        TestRequest::put()
            .uri("/api/torrent/upload/")
            .set_json(TorrentPutDTO {
                title: String::from("TEST TORRENT GET"),
                file_name: String::from("test_get"),
                nfo: None,
                uploaded_by: user.id,
                modded_by: None,
                payload: b"some random torrent bytes".to_vec(),
            }),
        &app,
    )
    .await;

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        read_body_json::<TorrentState, _>(res).await,
        TorrentState::UploadSuccess
    );

    let torrent = sqlx::query_as::<_, Torrent>("SELECT * FROM \"Torrent\" WHERE title = $1")
        .bind("TEST TORRENT GET")
        .fetch_one(&pool)
        .await
        .unwrap();

    let res = common::request_with_jwt(
        &login_res,
        TestRequest::get().uri(&format!("/api/torrent/{}", torrent.id)),
        &app,
    )
    .await;

    assert_eq!(res.status(), StatusCode::OK);

    assert_eq!(
        read_body_json::<UserState, _>(login_res).await,
        UserState::LoginSuccess { user: user.into() }
    );

    assert_eq!(
        read_body_json::<TorrentDTO, _>(res).await,
        torrent.into());

    common::teardown(pool).await;
}

#[actix_web::test]
#[ignore = "Not yet fully implemented"]
async fn test_torrent_get_with_info_hash() {
    let (pool, app) = common::setup().await;
    let login_res = common::register_and_login_new_user(
        RegisterDTO {
            username: String::from("test_torrent_get_with_info_hash"),
            email: String::from("test_torrent_get_with_info_hash"),
            password: String::from("test123"),
        },
        LoginDTO {
            username_or_email: String::from("test_torrent_get_with_info_hash"),
            password: String::from("test123"),
        },
        &app,
    )
    .await;
    assert_eq!(login_res.status(), StatusCode::OK);

    let user = sqlx::query_as::<_, User>("SELECT * FROM \"User\" WHERE username = $1")
        .bind("test_torrent_get_with_info_hash")
        .fetch_one(&pool)
        .await
        .unwrap();

    let res = common::request_with_jwt(
        &login_res,
        TestRequest::put()
            .uri("/api/torrent/upload/")
            .set_json(TorrentPutDTO {
                title: String::from("TEST TORRENT GET WITH INFO HASH"),
                file_name: String::from("test_get_with_info_hash"),
                nfo: None,
                uploaded_by: user.id,
                modded_by: None,
                payload: b"some random torrent bytes".to_vec(),
            }),
        &app,
    )
    .await;

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        read_body_json::<TorrentState, _>(res).await,
        TorrentState::UploadSuccess
    );

    let torrent = sqlx::query_as::<_, Torrent>("SELECT * FROM \"Torrent\" WHERE title = $1")
        .bind("TEST TORRENT GET WITH INFO HASH")
        .fetch_one(&pool)
        .await
        .unwrap();

    let res = common::request_with_jwt(
        &login_res,
        TestRequest::get()
            .uri(&format!("/api/torrent/{}", torrent.info_hash)),
        &app,
    )
    .await;

    assert_eq!(res.status(), StatusCode::OK);

    assert_eq!(
        read_body_json::<UserState, _>(login_res).await,
        UserState::LoginSuccess { user: user.into() }
    );

    assert_eq!(
        read_body_json::<TorrentDTO, _>(res).await,
        torrent.into());
}

#[actix_web::test]
#[ignore = "Not yet fully implemented"]
async fn test_torrent_get_with_filter() {
    let (pool, app) = common::setup().await;
    let login_res = common::register_and_login_new_user(
        RegisterDTO {
            username: String::from("test_torrent_get_with_filter"),
            email: String::from("test_torrent_get_with_filter"),
            password: String::from("test123"),
        },
        LoginDTO {
            username_or_email: String::from("test_torrent_get_with_filter"),
            password: String::from("test123"),
        },
        &app,
    )
    .await;
    assert_eq!(login_res.status(), StatusCode::OK);

    let user = sqlx::query_as::<_, User>("SELECT * FROM \"User\" WHERE username = $1")
        .bind("test_torrent_get_with_filter")
        .fetch_one(&pool)
        .await
        .unwrap();

    let res = common::request_with_jwt(
        &login_res,
        TestRequest::put()
            .uri("/api/torrent/upload/")
            .set_json(TorrentPutDTO {
                title: String::from("TEST TORRENT GET WITH FILTER"),
                file_name: String::from("test_get_with_filter"),
                nfo: None,
                uploaded_by: user.id,
                modded_by: None,
                payload: b"some random torrent bytes".to_vec(),
            }),
        &app,
    )
    .await;

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        read_body_json::<TorrentState, _>(res).await,
        TorrentState::UploadSuccess
    );

    let torrent = sqlx::query_as::<_, Torrent>("SELECT * FROM \"Torrent\" WHERE title = $1")
        .bind("TEST TORRENT GET WITH FILTER")
        .fetch_one(&pool)
        .await
        .unwrap();

    let res = common::request_with_jwt(
        &login_res,
        TestRequest::get()
            .uri("/api/torrent/")
            .set_json(TorrentFilter{
                uploaded_at_max: None,
                uploaded_at_min: None,
                uploaded_by: None,
                order_by: None,
                limit: None,
            }),
        &app,
    )
    .await;

    assert_eq!(res.status(), StatusCode::OK);

    assert_eq!(
        read_body_json::<UserState, _>(login_res).await,
        UserState::LoginSuccess { user: user.into() }
    );

    assert_eq!(
        read_body_json::<Vec<TorrentDTO>, _>(res).await,
        vec![torrent.into()]);
}
