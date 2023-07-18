use actix_multipart::Multipart;
use actix_web::{get, patch, put, web, HttpResponse};
use actix_web_validator::Json;

use laguna_backend_model::torrent::{Torrent, TorrentDTO, TorrentPatchDTO};

use futures::{StreamExt, TryStreamExt};
use sqlx::PgPool;

use uuid::Uuid;

use crate::error::{torrent::TorrentError, APIError};

/// `GET /api/torrent/{id}`
/// # Example
/// ## Request
/// ```sh
/// curl -X GET \
///      -i 'http://127.0.0.1:6969/api/torrent/00f045ac-1f4d-4601-b2e3-87476dc462e6'
///      -H 'X-Access-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg5OTMwNTksImlhdCI6MTY4ODk5Mjk5OSwiaWQiOiIwMGYwNDVhYy0xZjRkLTQ2MDEtYjJlMy04NzQ3NmRjNDYyZTYiLCJ1c2VybmFtZSI6InRlc3QiLCJmaXJzdF9sb2dpbiI6IjIwMjMtMDctMTBUMTI6NDI6MzIuMzk2NjQ3WiIsImxhc3RfbG9naW4iOiIyMDIzLTA3LTEwVDEyOjQzOjE5LjIxNjA0N1oiLCJhdmF0YXJfdXJsIjpudWxsLCJyb2xlIjoiTm9ybWllIiwiYmVoYXZpb3VyIjoiTHVya2VyIiwiaXNfYWN0aXZlIjp0cnVlLCJoYXNfdmVyaWZpZWRfZW1haWwiOmZhbHNlLCJpc19oaXN0b3J5X3ByaXZhdGUiOnRydWUsImlzX3Byb2ZpbGVfcHJpdmF0ZSI6dHJ1ZX0.izClLn6kANl2kpIv2QqzmKJy7tmpNZqKKvcd4RoGW_c' \
///      -H 'X-Refresh-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg0NjkzMzksImlhdCI6MTY4ODQ2NzUzOSwidXNlcm5hbWUiOiJ0ZXN0IiwiZW1haWwiOiJ0ZXN0QGxhZ3VuYS5pbyIsInBhc3N3b3JkIjoiZWNkNzE4NzBkMTk2MzMxNmE5N2UzYWMzNDA4Yzk4MzVhZDhjZjBmM2MxYmM3MDM1MjdjMzAyNjU1MzRmNzVhZSIsImZpcnN0X2xvZ2luIjoiMjAyMy0wNy0wNFQxMDoxODoxNy4zOTE2OThaIiwibGFzdF9sb2dpbiI6IjIwMjMtMDctMDRUMTA6MTg6MTcuMzkxNjk4WiIsImF2YXRhcl91cmwiOm51bGwsInJvbGUiOiJOb3JtaWUiLCJpc19hY3RpdmUiOnRydWUsImhhc192ZXJpZmllZF9lbWFpbCI6ZmFsc2UsImlzX2hpc3RvcnlfcHJpdmF0ZSI6dHJ1ZSwiaXNfcHJvZmlsZV9wcml2YXRlIjp0cnVlfQ.5fdMnIj0WqV0lszANlJD_x5-Oyq2h8bhqDkllz1CGg4'
/// ```
/// ## Response
/// HTTP/1.1 200 OK
/// ```json
/// {
///    "title": "test",
///    "file_name": "test_upload",
///    "nfo": null,
///    "info_hash": "aae8b4b6a0b9b6b5b4b6b5b4b6b5b4b6b5b4b6b5",
///    "uploaded_at": "2023-07-10T12:42:32.396647Z",
///    "uploaded_by": "00f045ac-1f4d-4601-b2e3-87476dc462e6",
///    "modded_by": null,
/// }
/// ```
#[get("/{id}")]
pub async fn get_torrent(
    id: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
    let torrent = sqlx::query_as::<_, Torrent>("SELECT * FROM \"Torrent\" WHERE id = $1")
        .bind(id.into_inner())
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| TorrentError::DoesNotExist)?;
    Ok(HttpResponse::Ok().json(TorrentDTO::from(torrent)))
}

/// `PATCH /api/torrent/`
/// # Example
/// ## Request
/// ```sh
/// curl -X PATCH \
///      -i 'http://127.0.0.1:6969/api/torrent/' \
///      -H 'Content-Type: application/json' \
///      -H 'X-Access-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg5OTMwNTksImlhdCI6MTY4ODk5Mjk5OSwiaWQiOiIwMGYwNDVhYy0xZjRkLTQ2MDEtYjJlMy04NzQ3NmRjNDYyZTYiLCJ1c2VybmFtZSI6InRlc3QiLCJmaXJzdF9sb2dpbiI6IjIwMjMtMDctMTBUMTI6NDI6MzIuMzk2NjQ3WiIsImxhc3RfbG9naW4iOiIyMDIzLTA3LTEwVDEyOjQzOjE5LjIxNjA0N1oiLCJhdmF0YXJfdXJsIjpudWxsLCJyb2xlIjoiTm9ybWllIiwiYmVoYXZpb3VyIjoiTHVya2VyIiwiaXNfYWN0aXZlIjp0cnVlLCJoYXNfdmVyaWZpZWRfZW1haWwiOmZhbHNlLCJpc19oaXN0b3J5X3ByaXZhdGUiOnRydWUsImlzX3Byb2ZpbGVfcHJpdmF0ZSI6dHJ1ZX0.izClLn6kANl2kpIv2QqzmKJy7tmpNZqKKvcd4RoGW_c' \
///      -H 'X-Refresh-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg0NjkzMzksImlhdCI6MTY4ODQ2NzUzOSwidXNlcm5hbWUiOiJ0ZXN0IiwiZW1haWwiOiJ0ZXN0QGxhZ3VuYS5pbyIsInBhc3N3b3JkIjoiZWNkNzE4NzBkMTk2MzMxNmE5N2UzYWMzNDA4Yzk4MzVhZDhjZjBmM2MxYmM3MDM1MjdjMzAyNjU1MzRmNzVhZSIsImZpcnN0X2xvZ2luIjoiMjAyMy0wNy0wNFQxMDoxODoxNy4zOTE2OThaIiwibGFzdF9sb2dpbiI6IjIwMjMtMDctMDRUMTA6MTg6MTcuMzkxNjk4WiIsImF2YXRhcl91cmwiOm51bGwsInJvbGUiOiJOb3JtaWUiLCJpc19hY3RpdmUiOnRydWUsImhhc192ZXJpZmllZF9lbWFpbCI6ZmFsc2UsImlzX2hpc3RvcnlfcHJpdmF0ZSI6dHJ1ZSwiaXNfcHJvZmlsZV9wcml2YXRlIjp0cnVlfQ.5fdMnIj0WqV0lszANlJD_x5-Oyq2h8bhqDkllz1CGg4' \
///      --data '{
///         "id": "00f045ac-1f4d-4601-b2e3-87476dc462e6",
///         "title": "TEST (2020)",
///         "file_name": "test_upload",
///         "nfo": null,
///         "modded_by": null
///      }'
/// ```
/// ## Response
/// HTTP/1.1 200 OK
/// ```json
/// {
///   "id": "00f045ac-1f4d-4601-b2e3-87476dc462e6",
///   "title": "TEST (2020)",
///   "file_name": "test_upload",
///   "nfo": null,
///   "info_hash": "aae8b4b6a0b9b6b5b4b6b5b4b6b5b4b6b5b4b6b5",
///   "uploaded_at": "2023-07-10T12:42:32.396647Z",
///   "uploaded_by": "ffff45ac-1f4d-46f1-b2e3-87476dc462e6",
///   "modded_by": null,
/// }
/// ```
/// Returns updated [`TorrentDTO`].
#[patch("/")]
pub async fn patch_torrent(
    torrent_dto: Json<TorrentPatchDTO>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
    let torrent_dto = TorrentDTO::from(
        sqlx::query_as::<_, Torrent>(
            r#"
    UPDATE "Torrent" 
    SET title = $1, file_name = $2, nfo = $3, modded_by = $4
    WHERE id = $5
    RETURNING *
    "#,
        )
        .bind(&torrent_dto.file_name)
        .bind(&torrent_dto.nfo)
        .bind(&torrent_dto.modded_by)
        .bind(&torrent_dto.title)
        .bind(&torrent_dto.id)
        .fetch_one(pool.get_ref())
        .await?,
    );
    Ok(HttpResponse::Ok().json(torrent_dto))
}

/// `PUT /api/torrent/`
/// # Example
/// ## Request
/// ```sh
/// curl -X PUT \
///      -i 'http://127.0.0.1:6969/api/torrent/' \
///      -H 'X-Access-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg5OTMwNTksImlhdCI6MTY4ODk5Mjk5OSwiaWQiOiIwMGYwNDVhYy0xZjRkLTQ2MDEtYjJlMy04NzQ3NmRjNDYyZTYiLCJ1c2VybmFtZSI6InRlc3QiLCJmaXJzdF9sb2dpbiI6IjIwMjMtMDctMTBUMTI6NDI6MzIuMzk2NjQ3WiIsImxhc3RfbG9naW4iOiIyMDIzLTA3LTEwVDEyOjQzOjE5LjIxNjA0N1oiLCJhdmF0YXJfdXJsIjpudWxsLCJyb2xlIjoiTm9ybWllIiwiYmVoYXZpb3VyIjoiTHVya2VyIiwiaXNfYWN0aXZlIjp0cnVlLCJoYXNfdmVyaWZpZWRfZW1haWwiOmZhbHNlLCJpc19oaXN0b3J5X3ByaXZhdGUiOnRydWUsImlzX3Byb2ZpbGVfcHJpdmF0ZSI6dHJ1ZX0.izClLn6kANl2kpIv2QqzmKJy7tmpNZqKKvcd4RoGW_c' \
///      -H 'X-Refresh-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg0NjkzMzksImlhdCI6MTY4ODQ2NzUzOSwidXNlcm5hbWUiOiJ0ZXN0IiwiZW1haWwiOiJ0ZXN0QGxhZ3VuYS5pbyIsInBhc3N3b3JkIjoiZWNkNzE4NzBkMTk2MzMxNmE5N2UzYWMzNDA4Yzk4MzVhZDhjZjBmM2MxYmM3MDM1MjdjMzAyNjU1MzRmNzVhZSIsImZpcnN0X2xvZ2luIjoiMjAyMy0wNy0wNFQxMDoxODoxNy4zOTE2OThaIiwibGFzdF9sb2dpbiI6IjIwMjMtMDctMDRUMTA6MTg6MTcuMzkxNjk4WiIsImF2YXRhcl91cmwiOm51bGwsInJvbGUiOiJOb3JtaWUiLCJpc19hY3RpdmUiOnRydWUsImhhc192ZXJpZmllZF9lbWFpbCI6ZmFsc2UsImlzX2hpc3RvcnlfcHJpdmF0ZSI6dHJ1ZSwiaXNfcHJvZmlsZV9wcml2YXRlIjp0cnVlfQ.5fdMnIj0WqV0lszANlJD_x5-Oyq2h8bhqDkllz1CGg4' \
///      -H 'Content-Type: application/json' \
///     --data '{
///        "title": "test",
///        "file_name": "test_upload",
///        "nfo": null,
///        "uploaded_by": "00f045ac-1f4d-4601-b2e3-87476dc462e6",
///        "modded_by": null,
///     }'
/// ```
/// ## Response
/// 1. On upload success: HTTP/1.1 200 OK
/// 2. If torrent already exists: HTTP/1.1 208 Already Reported
/// TODO: Right now, we send it via Json body, but we should use multipart/form-data.
#[put("/")]
pub async fn put_torrent(
    mut payload: Multipart,
    _pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
    if let Some(mut field) = payload.try_next().await? {
        // TODO: Check for Content-Type: application/x-bittorrent
        let mut buf = Vec::new();
        while let Some(chunk) = field.next().await {
            buf.extend_from_slice(&chunk?);
        }
    }

    /*

    let torrent = sqlx::query_as::<_, Torrent>("SELECT * FROM \"Torrent\" WHERE title = $1")
        .bind(&torrent_dto.title)
        .fetch_optional(pool.get_ref())
        .await?;

    match torrent {
        Some(_) => Ok(HttpResponse::AlreadyReported().finish()),
        None => {
            let torrent = sqlx::query_as::<_, Torrent>(
                r#"
            INSERT INTO "Torrent" (title, file_name, nfo, info_hash, uploaded_at, uploaded_by)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING *
                "#,
            )
            .bind(&torrent_dto.title)
            .bind(&torrent_dto.file_name)
            .bind(&torrent_dto.nfo)
            .bind(format!("{:x}", Sha256::digest(&torrent_dto.payload))) // TODO: Hash only info section of Torrent. This is fine for now, but redundant.
            .bind(Utc::now())
            .bind(&torrent_dto.uploaded_by)
            .fetch_one(pool.get_ref())
            .await?;

            Ok(HttpResponse::Ok().finish())
        }
    }
    */
    todo!()
}
