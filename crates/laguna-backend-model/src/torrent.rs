use crate::genre::Genre;
use crate::speedlevel::SpeedLevel;
use chrono::{DateTime, Utc};

use derivative::Derivative;
use serde::{Deserialize, Serialize};

use laguna_backend_tracker_common::info_hash::InfoHash;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::consts::TORRENT_FILENAME_MAX_LEN;
use crate::consts::TORRENT_FILENAME_MIN_LEN;

use sqlx::FromRow;
use validator::Validate;

/// Torrent DB object.
#[derive(Derivative, Serialize, Deserialize, PartialEq, Eq, Clone, FromRow, Validate, ToSchema)]
#[derivative(Debug)]
pub struct Torrent<const N: usize> {
  /// Torrent's info hash, unique for all torrents, sha1 or sha256 of [`laguna_backend_dto::torrent::TorrentFile`]'s bencoded info section.
  pub info_hash: InfoHash<N>,
  /// Raw torrent file's bytes.
  #[derivative(Debug = "ignore")]
  pub raw: Vec<u8>,
  /// Announce URL of torrent file.
  pub announce_url: Option<String>,
  /// Length of torrent file in bytes.
  pub length: i64,
  /// File name of torrent file.
  #[validate(
    non_control_character,
    length(min = "TORRENT_FILENAME_MIN_LEN", max = "TORRENT_FILENAME_MAX_LEN")
  )]
  pub file_name: String,
  /// NFO string, torrent file description.
  /// This attribute doesn't come with torrent file, but is determined on FE when torrent file is uploaded.
  pub nfo: Option<String>,
  /// Is this torrent freeleech-able?
  /// If torrent is freeleech, then `hnr_count` of [`User`] won't go up in case of H&R event happening.
  /// Freeleech torrents allow for as many as leeches as possible, this is marked on FE when torrent file is uploaded.
  pub is_freeleech: bool,
  /// Genre is marked on FE on upload, if genre is [`None`] then this torrent has no genre.
  /// For example software tools might be genre-less.
  pub genre: Option<Genre>,
  /// Number of leeches in this torrent's active swarm.
  pub leech_count: i32,
  /// Number of seeds in this torrent's active swarm.
  pub seed_count: i32,
  /// Number of times the torrent's file has been completed.
  pub completed_count: i32,
  /// How fast is download of torrent's file (content the torrent file is pointing to).
  /// This is computed realtime depending on how many seeds and leeches in network, as well as transmission speed.
  /// It is not 100% accurate.
  pub speedlevel: SpeedLevel,
  /// When was this torrent file created? Determined by whoever created the torrent file.
  pub creation_date: DateTime<Utc>,
  /// Who created this torrent file?
  pub created_by: Option<String>,
  /// When was this torrent file uploaded? Determined by BE.
  pub uploaded_at: DateTime<Utc>,
  /// Who uploaded this torrent file? Determined by BE.
  pub uploaded_by: Uuid,
  /// When was this torrent file modded? Determined by BE.
  /// Note that all moderation actions are not stored, just the latest.
  pub modded_at: Option<DateTime<Utc>>,
  /// Who moderated this torrent file? Determined by BE.
  /// Note that all moderators moderating this file are not stored, just the latest.
  pub modded_by: Option<Uuid>,
}
