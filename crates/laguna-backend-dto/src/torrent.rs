use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
#[cfg(feature = "testx")]
use fake::Dummy;
use laguna_backend_model::consts::{TORRENT_FILENAME_MAX_LEN, TORRENT_FILENAME_MIN_LEN};
use laguna_backend_model::consts::{TORRENT_TITLE_MAX_LEN, TORRENT_TITLE_MIN_LEN};
use laguna_backend_model::genre::Genre;
use laguna_backend_model::speedlevel::SpeedLevel;
use laguna_backend_model::torrent::Torrent;

use actix_multipart_extract::File as ActixFile;
use actix_multipart_extract::MultipartForm;
use serde::{Deserialize, Serialize};

use serde_bytes::ByteBuf;

use utoipa::ToSchema;
use validator::Validate;

pub type TorrentDTO = Torrent;

#[derive(Debug, Deserialize, MultipartForm, ToSchema)]
pub struct TorrentPutDTO {
  #[multipart(max_size = 1MB)]
  pub torrent: ActixFile,
}

impl TryFrom<&ActixFile> for TorrentFile {
  type Error = serde_bencode::Error;

  fn try_from(actix_file: &ActixFile) -> Result<Self, Self::Error> {
    serde_bencode::de::from_bytes::<TorrentFile>(&actix_file.bytes)
  }
}

/// Torrent file (raw).
/// Some specifications:
/// <http://bittorrent.org/beps/bep_0052.html>
/// <http://bittorrent.org/beps/bep_0023.html>
/// <http://bittorrent.org/beps/bep_0007.html>
/// More examples of torrent files:
/// <https://github.com/webtorrent/webtorrent-fixtures/tree/master>
/// <https://chocobo1.github.io/bencode_online> (see example)
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Validate)]
// #[cfg_attr(feature = "testx", derive(Dummy))]
pub struct TorrentFile {
  // announce is set by torrent client
  #[serde(rename = "announce")]
  #[serde(default)]
  pub announce_url: Option<String>,
  // announce-list is set by torrent client
  #[serde(rename = "accounce-list")]
  pub announce_list: Option<Vec<Vec<String>>>,
  // title is set by FE
  #[validate(length(min = "TORRENT_TITLE_MIN_LEN", max = "TORRENT_TITLE_MAX_LEN"))]
  #[serde(default)]
  pub title: Option<String>,
  // nfo is set by FE
  #[validate(length(min = "TORRENT_FILENAME_MIN_LEN", max = "TORRENT_FILENAME_MAX_LEN"))]
  #[serde(default)]
  pub nfo: Option<String>,
  // comment is set by torrent client
  #[serde(default)]
  pub comment: Option<String>,
  // encoding is set by torrent client, we deny all except UTF-8
  #[serde(default)]
  pub encoding: Option<String>,
  // this is a timestamp, creation date is set by torrent client
  #[serde(rename = "creation date", with = "ts_seconds")]
  pub creation_date: DateTime<Utc>,
  // created by is set by torrent client
  #[serde(rename = "created by")]
  #[serde(default)]
  pub created_by: Option<String>,
  // info is set by torrent client
  pub info: TorrentInfo,
  // url-list is set by torrent client (this is webtorrent specific)
  #[serde(rename = "url-list")]
  #[serde(default)]
  pub url_list: Option<Vec<String>>,
  // website is set by torrent client (this is webtorrent specific)
  #[serde(default)]
  pub website: Option<String>,
  // nodes is set by torrent client
  #[serde(default)]
  pub nodes: Option<Vec<Node>>,
  // httpseeds is set by torrent client
  #[serde(default)]
  pub httpseeds: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
// #[cfg_attr(feature = "testx", derive(Dummy))]
pub struct TorrentInfo {
  #[serde(rename = "file-duration")]
  #[serde(default)]
  pub file_duration: Option<Vec<i32>>,
  #[serde(rename = "file-media")]
  #[serde(default)]
  pub file_media: Option<Vec<i32>>,
  // length of single file if Torrent describes a single file
  // if torrent describes directory, then lengths can be found in [`File`].
  #[serde(default)]
  pub length: Option<i64>,
  // Name of single file or root directory if directory.
  pub name: String,
  #[serde(rename = "piece length")]
  pub piece_length: i64,
  #[serde(rename = "pieces")]
  pub pieces: ByteBuf,
  #[serde(rename = "root hash")]
  #[serde(default)]
  pub root_hash: Option<String>,
  #[serde(default)]
  pub md5sum: Option<String>,
  #[serde(default)]
  pub private: Option<u8>,
  #[serde(default)]
  pub files: Option<Vec<File>>,
  #[serde(default)]
  pub profiles: Option<Vec<TorrentProfile>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "testx", derive(Dummy))]
pub struct File {
  pub length: i32,
  pub path: Vec<String>,
  #[serde(default)]
  pub md5sum: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Node(String, i32);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "testx", derive(Dummy))]
pub struct TorrentProfile {
  #[serde(default)]
  pub acodec: Option<String>,
  pub height: i32,
  #[serde(default)]
  pub vcodec: Option<String>,
  pub width: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Validate, ToSchema)]
pub struct TorrentPatchDTO {
  #[serde(default)]
  pub nfo: Option<String>,
  #[serde(default)]
  pub genre: Option<Genre>,
}
