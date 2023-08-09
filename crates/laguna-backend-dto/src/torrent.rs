use chrono::{DateTime, Utc};
#[cfg(feature = "testx")]
use fake::Dummy;
use laguna_backend_model::consts::{TORRENT_FILENAME_MAX_LEN, TORRENT_FILENAME_MIN_LEN};
use laguna_backend_model::consts::{TORRENT_TITLE_MAX_LEN, TORRENT_TITLE_MIN_LEN};
use laguna_backend_model::speedlevel::SpeedLevel;
use laguna_backend_model::torrent::Torrent;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Validate)]
pub struct TorrentDTO {
    pub id: Uuid,
    pub announce_url: String,
    pub length: i32,
    #[validate(
        non_control_character,
        length(min = "TORRENT_TITLE_MIN_LEN", max = "TORRENT_TITLE_MAX_LEN")
    )]
    pub title: String,
    #[validate(
        non_control_character,
        length(min = "TORRENT_FILENAME_MIN_LEN", max = "TORRENT_FILENAME_MAX_LEN")
    )]
    pub file_name: String,
    pub nfo: Option<String>,
    pub leech_count: i32,
    pub seed_count: i32,
    pub completed_count: i32,
    pub speedlevel: SpeedLevel,
    pub info_hash: String,
    pub uploaded_at: DateTime<Utc>,
    pub uploaded_by: Uuid,
    pub modded_at: Option<DateTime<Utc>>,
    pub modded_by: Option<Uuid>,
}

impl From<Torrent> for TorrentDTO {
    fn from(torrent: Torrent) -> Self {
        Self {
            id: torrent.id,
            announce_url: torrent.announce_url,
            length: torrent.length,
            title: torrent.title,
            file_name: torrent.file_name,
            nfo: torrent.nfo,
            leech_count: torrent.leech_count,
            seed_count: torrent.seed_count,
            completed_count: torrent.completed_count,
            speedlevel: torrent.speedlevel,
            info_hash: torrent.info_hash,
            uploaded_at: torrent.uploaded_at,
            uploaded_by: torrent.uploaded_by,
            modded_at: torrent.modded_at,
            modded_by: torrent.modded_by,
        }
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
pub struct TorrentPutDTO {
    // announce is set by torrent client
    #[serde(rename = "announce")]
    pub announce_url: Option<String>,
    // announce-list is set by torrent client
    #[serde(rename = "accounce-list")]
    pub announce_list: Option<Vec<Vec<String>>>,
    // title is set by FE
    #[validate(length(min = "TORRENT_TITLE_MIN_LEN", max = "TORRENT_TITLE_MAX_LEN"))]
    pub title: Option<String>,
    // nfo is set by FE
    #[validate(length(min = "TORRENT_FILENAME_MIN_LEN", max = "TORRENT_FILENAME_MAX_LEN"))]
    pub nfo: Option<String>,
    // speedlevel is set by FE
    pub speedlevel: Option<SpeedLevel>,
    // comment is set by torrent client
    pub comment: Option<String>,
    // encoding is set by torrent client, we deny all except UTF-8
    pub encoding: Option<String>,
    // creation date is set by torrent client
    #[serde(rename = "creation date")]
    pub creation_date: i32,
    // created by is set by torrent client
    #[serde(rename = "created by")]
    pub created_by: Option<String>,
    // info is set by torrent client
    pub info: TorrentPutInfoDTO,
    // url-list is set by torrent client (this is webtorrent specific)
    #[serde(rename = "url-list")]
    pub url_list: Option<Vec<String>>,
    // website is set by torrent client (this is webtorrent specific)
    pub website: Option<String>,
    // nodes is set by torrent client
    pub nodes: Option<Vec<Node>>,
    // httpseeds is set by torrent client
    pub httpseeds: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
// #[cfg_attr(feature = "testx", derive(Dummy))]
pub struct TorrentPutInfoDTO {
    #[serde(rename = "file-duration")]
    pub file_duration: Option<Vec<i32>>,
    #[serde(rename = "file-media")]
    pub file_media: Option<Vec<i32>>,
    pub length: i32,
    pub name: String,
    #[serde(rename = "piece length")]
    pub piece_length: i32,
    #[serde(rename = "pieces")]
    pub pieces: ByteBuf,
    #[serde(rename = "root hash")]
    pub root_hash: Option<String>,
    pub md5sum: Option<String>,
    pub private: Option<u8>,
    pub files: Option<Vec<File>>,
    pub profiles: Option<Vec<TorrentPutInfoProfileDTO>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "testx", derive(Dummy))]
pub struct File {
    pub length: i32,
    pub path: Vec<String>,
    pub md5sum: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Node(String, i32);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "testx", derive(Dummy))]
pub struct TorrentPutInfoProfileDTO {
    pub acodec: Option<String>,
    pub height: i32,
    pub vcodec: Option<String>,
    pub width: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Validate)]
pub struct TorrentPatchDTO {
    pub id: Uuid,
    #[validate(length(min = "TORRENT_TITLE_MIN_LEN", max = "TORRENT_TITLE_MAX_LEN"))]
    pub title: String,
    #[validate(length(min = "TORRENT_FILENAME_MIN_LEN", max = "TORRENT_FILENAME_MAX_LEN"))]
    pub file_name: String,
    pub nfo: Option<String>,
}
