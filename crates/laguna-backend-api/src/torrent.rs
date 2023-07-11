use std::{fs::File, path::PathBuf};

use actix_files::NamedFile;
use actix_web::{get, put, web, HttpResponse};
use chrono::{DateTime, Utc};
use laguna_backend_middleware::filters::torrent::{TorrentFilter, DEFAULT_TORRENT_FILTER_LIMIT};
use laguna_backend_model::{
    torrent::{Torrent, TorrentDTO},
    user::UserDTO,
};
use sqlx::PgPool;
use std::io::Write;
use uuid::Uuid;

use crate::{
    error::{APIError, TorrentError},
    state::TorrentState,
};

/// `GET /api/torrent/{id}`
/// This only gets you torrent metadata (stored in DB).
#[get("/{id}")]
pub async fn get_torrent(
    id: web::Path<Uuid>,
    _user: UserDTO,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
    let torrent = sqlx::query_as::<_, Torrent>("SELECT * FROM \"Torrent\" WHERE id = $1")
        .bind(id.into_inner())
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| TorrentError::DoesNotExist)?;
    Ok(HttpResponse::Ok().json(TorrentDTO::from(torrent)))
}

/// `GET /api/torrent/{info_hash}`
/// This only gets you torrent metadata (stored in DB).
#[get("/{info_hash}")]
pub async fn get_torrent_by_info_hash(
    info_hash: web::Path<String>,
    _user: UserDTO,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
    let torrent = sqlx::query_as::<_, Torrent>("SELECT * FROM \"Torrent\" WHERE info_hash = $1")
        .bind(info_hash.into_inner())
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| TorrentError::DoesNotExist)?;
    Ok(HttpResponse::Ok().json(TorrentDTO::from(torrent)))
}

/// `GET /api/torrent`
#[get("/")]
pub async fn get_torrents_filtered(
    filter: web::Json<TorrentFilter>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
    // Dynamic query generation is still being worked on: https://github.com/launchbadge/sqlx/issues/291
    // See: https://github.com/launchbadge/sqlx/issues/364
    let torrents = sqlx::query_as::<_, Torrent>(&format!(
        r#"
        SELECT * 
        FROM "Torrent" 
        INNER JOIN "User" USING (id)
        WHERE 
        (uploaded_at >= $1 AND uploaded_at <= $2) AND
        (($3 IS NULL and username IS NULL) OR username = $3)
        {order_by}
        LIMIT $4
        "#,
        order_by = match filter.order_by {
            None => String::from(""),
            Some(ref order_by) => order_by.to_string(),
        }
    ))
    .bind(
        filter
            .uploaded_at_min
            .unwrap_or_else(|| DateTime::<Utc>::MIN_UTC),
    )
    .bind(
        filter
            .uploaded_at_max
            .unwrap_or_else(|| DateTime::<Utc>::MAX_UTC),
    )
    .bind(&filter.uploaded_by)
    .bind(filter.limit.unwrap_or_else(|| DEFAULT_TORRENT_FILTER_LIMIT))
    .fetch_all(pool.get_ref())
    .await?;
    Ok(HttpResponse::Ok().json(
        torrents
            .into_iter()
            .map(|torrent| TorrentDTO::from(torrent))
            .collect::<Vec<TorrentDTO>>(),
    ))
}

/// `GET /api/torrent/download/{id}`
/// Actually downloads .torrent with file name {id}.torrent.
/// But when downloading renames it to {file_name}.torrent.
/// We store files in `torrents/` folder on localfs in {id}.torrent format to ensure uniqueness.
/// TODO: Use magnet links.
#[get("/{id}")]
pub async fn get_torrent_download(
    id: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> Result<NamedFile, APIError> {
    let file_name =
        sqlx::query_scalar::<_, String>("SELECT file_name FROM \"Torrent\" WHERE id = $1")
            .bind(id.into_inner())
            .fetch_optional(pool.get_ref())
            .await?
            .ok_or_else(|| TorrentError::DoesNotExist)?;
    Ok(NamedFile::open_async(PathBuf::from(format!("torrents/{}.torrent", file_name))).await?)
}

/// `PUT /api/torrent/upload`
/// TODO: Right now, we send it via Json body, but we should use multipart/form-data.
#[put("/")]
pub async fn put_torrent(
    torrent_dto: web::Json<TorrentDTO>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
    let torrent = sqlx::query_as::<_, Torrent>("SELECT * FROM \"Torrent\" WHERE id = $1")
        .bind(torrent_dto.id)
        .fetch_optional(pool.get_ref())
        .await?;

    match torrent {
        Some(_) => Ok(HttpResponse::AlreadyReported().json(TorrentState::AlreadyExists)),
        None => {
            let mut transaction = pool.begin().await?;
            // Transaction is rolled back if file creation fails.
            let mut file = File::create(format!("/torrents/{}.torrent", torrent_dto.id))?;
            file.write_all(&torrent_dto.payload)?;

            // Transaction is rolled back if file create time cannot be fetched.
            let file_create_time = file.metadata().map(|metadata| metadata.created())??;

            sqlx::query(
                r#"
            INSERT INTO "Torrent" (name, file_name, nfo, info_hash, uploaded_at, uploaded_by) 
                VALUES ($1, $2, $3, $4, $5, $6)
                "#,
            )
            .bind(&torrent_dto.name)
            .bind(&torrent_dto.file_name)
            .bind(&torrent_dto.nfo)
            .bind(DateTime::<Utc>::from(file_create_time))
            .bind(&torrent_dto.uploaded_by)
            .execute(&mut transaction)
            .await?;

            transaction.commit().await?;
            Ok(HttpResponse::Ok().json(TorrentState::UploadSuccess))
        }
    }
}
