use bendy::decoding::{self, FromBencode, ResultExt};
use bendy::encoding::{self, AsString, ToBencode};
use chrono::serde::ts_seconds;
use chrono::{DateTime, NaiveDateTime, Utc};
#[cfg(feature = "testx")]
use fake::Dummy;
use laguna_backend_model::consts::{TORRENT_FILENAME_MAX_LEN, TORRENT_FILENAME_MIN_LEN};
use laguna_backend_model::consts::{TORRENT_TITLE_MAX_LEN, TORRENT_TITLE_MIN_LEN};
use laguna_backend_model::genre::Genre;

use laguna_backend_model::torrent::Torrent;

use actix_multipart_extract::File as ActixFile;
use actix_multipart_extract::MultipartForm;
use serde::{Deserialize, Serialize};

use utoipa::ToSchema;
use validator::Validate;

pub type TorrentDTO = Torrent;

#[derive(Debug, Deserialize, MultipartForm, ToSchema)]
pub struct TorrentPutDTO {
  #[multipart(max_size = 1MB)]
  pub torrent: ActixFile,
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

impl FromBencode for TorrentFile {
  fn decode_bencode_object(object: bendy::decoding::Object) -> Result<Self, bendy::decoding::Error>
  where
    Self: Sized,
  {
    let mut dict = object.try_into_dictionary()?;
    let mut announce_url = None;
    let mut announce_list = None;
    let mut title = None;
    let mut nfo = None;
    let mut comment = None;
    let mut encoding = None;
    let mut cration_date = None;
    let mut created_by = None;
    let mut info = None;
    let mut url_list = None;
    let mut website = None;
    let mut nodes = None;
    let mut httpseeds = None;
    while let Some(pair) = dict.next_pair()? {
      match pair {
        (b"announce", value) => {
          announce_url = String::decode_bencode_object(value)
            .context("announce")
            .map(Some)?;
        },
        (b"announce-list", value) => {
          announce_list = Vec::<Vec<String>>::decode_bencode_object(value)
            .context("announce-list")
            .map(Some)?;
        },
        (b"title", value) => {
          title = String::decode_bencode_object(value)
            .context("title")
            .map(Some)?;
        },
        (b"nfo", value) => {
          nfo = String::decode_bencode_object(value)
            .context("nfo")
            .map(Some)?;
        },
        (b"comment", value) => {
          comment = String::decode_bencode_object(value)
            .context("comment")
            .map(Some)?;
        },
        (b"encoding", value) => {
          encoding = String::decode_bencode_object(value)
            .context("encoding")
            .map(Some)?;
        },
        (b"creation date", value) => {
          cration_date = i64::decode_bencode_object(value)
            .context("creation date")
            .map(|i| {
              DateTime::<Utc>::from_naive_utc_and_offset(
                NaiveDateTime::from_timestamp_opt(i, 0).unwrap(),
                Utc,
              )
            })
            .map(Some)?;
        },
        (b"created by", value) => {
          created_by = String::decode_bencode_object(value)
            .context("created by")
            .map(Some)?;
        },
        (b"info", value) => {
          info = TorrentInfo::decode_bencode_object(value)
            .context("info")
            .map(Some)?;
        },
        (b"url-list", value) => {
          url_list = Vec::<String>::decode_bencode_object(value)
            .context("url-list")
            .map(Some)?;
        },
        (b"website", value) => {
          website = String::decode_bencode_object(value)
            .context("website")
            .map(Some)?;
        },
        (b"nodes", value) => {
          nodes = Vec::<Node>::decode_bencode_object(value)
            .context("nodes")
            .map(Some)?;
        },
        (b"httpseeds", value) => {
          httpseeds = Vec::<String>::decode_bencode_object(value)
            .context("httpseeds")
            .map(Some)?;
        },
        (unknown_field, _) => {
          return Err(decoding::Error::unexpected_field(String::from_utf8_lossy(
            unknown_field,
          )));
        },
      }
    }
    Ok(Self {
      announce_url,
      announce_list,
      title,
      nfo,
      comment,
      encoding,
      creation_date: cration_date.ok_or_else(|| decoding::Error::missing_field("creation date"))?,
      created_by,
      info: info.ok_or_else(|| decoding::Error::missing_field("info"))?,
      url_list,
      website,
      nodes,
      httpseeds,
    })
  }
}

impl ToBencode for TorrentFile {
  const MAX_DEPTH: usize = 10;
  fn encode(&self, encoder: encoding::SingleItemEncoder) -> Result<(), encoding::Error> {
    encoder.emit_unsorted_dict(|d| {
      if let Some(ref announce_url) = self.announce_url {
        d.emit_pair(b"announce", announce_url)?;
      }
      if let Some(ref announce_list) = self.announce_list {
        d.emit_pair(b"announce-list", announce_list)?;
      }
      if let Some(ref title) = self.title {
        d.emit_pair(b"title", title)?;
      }
      if let Some(ref nfo) = self.nfo {
        d.emit_pair(b"nfo", nfo)?;
      }
      if let Some(ref comment) = self.comment {
        d.emit_pair(b"comment", comment)?;
      }
      if let Some(ref encoding) = self.encoding {
        d.emit_pair(b"encoding", encoding)?;
      }
      d.emit_pair(b"creation date", self.creation_date.timestamp())?;
      if let Some(ref created_by) = self.created_by {
        d.emit_pair(b"created by", created_by)?;
      }
      d.emit_pair(b"info", &self.info)?;
      if let Some(ref url_list) = self.url_list {
        d.emit_pair(b"url-list", url_list)?;
      }
      if let Some(ref website) = self.website {
        d.emit_pair(b"website", website)?;
      }
      if let Some(ref nodes) = self.nodes {
        d.emit_pair(b"nodes", nodes)?;
      }
      if let Some(ref httpseeds) = self.httpseeds {
        d.emit_pair(b"httpseeds", httpseeds)?;
      }
      Ok(())
    })
  }
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
  pub pieces: Vec<u8>,
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

impl FromBencode for TorrentInfo {
  fn decode_bencode_object(object: decoding::Object) -> Result<Self, decoding::Error>
  where
    Self: Sized,
  {
    let mut dict = object.try_into_dictionary()?;
    let mut file_duration = None;
    let mut file_media = None;
    let mut length = None;
    let mut name = None;
    let mut piece_length = None;
    let mut pieces = None;
    let mut root_hash = None;
    let mut md5sum = None;
    let mut private = None;
    let mut files = None;
    let mut profiles = None;
    while let Some(pair) = dict.next_pair()? {
      match pair {
        (b"file-duration", value) => {
          file_duration = Vec::<i32>::decode_bencode_object(value)
            .context("file-duration")
            .map(Some)?;
        },
        (b"file-media", value) => {
          file_media = Vec::<i32>::decode_bencode_object(value)
            .context("file-media")
            .map(Some)?;
        },
        (b"length", value) => {
          length = i64::decode_bencode_object(value)
            .context("length")
            .map(Some)?;
        },
        (b"name", value) => {
          name = String::decode_bencode_object(value)
            .context("name")
            .map(Some)?;
        },
        (b"piece length", value) => {
          piece_length = i64::decode_bencode_object(value)
            .context("piece length")
            .map(Some)?;
        },
        (b"pieces", value) => {
          pieces = AsString::decode_bencode_object(value)
            .context("pieces")
            .map(|bytes| Some(bytes.0))?;
        },
        (b"root hash", value) => {
          root_hash = String::decode_bencode_object(value)
            .context("root hash")
            .map(Some)?;
        },
        (b"md5sum", value) => {
          md5sum = String::decode_bencode_object(value)
            .context("md5sum")
            .map(Some)?;
        },
        (b"private", value) => {
          private = u8::decode_bencode_object(value)
            .context("private")
            .map(Some)?;
        },
        (b"files", value) => {
          files = Vec::<File>::decode_bencode_object(value)
            .context("files")
            .map(Some)?;
        },
        (b"profiles", value) => {
          profiles = Vec::<TorrentProfile>::decode_bencode_object(value)
            .context("profiles")
            .map(Some)?;
        },
        (unknown_field, _) => {
          return Err(decoding::Error::unexpected_field(String::from_utf8_lossy(
            unknown_field,
          )));
        },
      }
    }
    Ok(Self {
      file_duration,
      file_media,
      length,
      name: name.ok_or_else(|| decoding::Error::missing_field("name"))?,
      piece_length: piece_length.ok_or_else(|| decoding::Error::missing_field("piece length"))?,
      pieces: pieces.ok_or_else(|| decoding::Error::missing_field("pieces"))?,
      root_hash,
      md5sum,
      private,
      files,
      profiles,
    })
  }
}

impl ToBencode for TorrentInfo {
  const MAX_DEPTH: usize = 10;
  fn encode(&self, encoder: encoding::SingleItemEncoder) -> Result<(), encoding::Error> {
    encoder.emit_unsorted_dict(|d| {
      if let Some(ref file_duration) = self.file_duration {
        d.emit_pair(b"file-duration", file_duration)?;
      }
      if let Some(ref file_media) = self.file_media {
        d.emit_pair(b"file-media", file_media)?;
      }
      if let Some(ref length) = self.length {
        d.emit_pair(b"length", length)?;
      }
      d.emit_pair(b"name", &self.name)?;
      d.emit_pair(b"piece length", self.piece_length)?;
      d.emit_pair(b"pieces", AsString(&self.pieces))?;
      if let Some(ref root_hash) = self.root_hash {
        d.emit_pair(b"root hash", root_hash)?;
      }
      if let Some(ref md5sum) = self.md5sum {
        d.emit_pair(b"md5sum", md5sum)?;
      }
      if let Some(ref private) = self.private {
        d.emit_pair(b"private", private)?;
      }
      if let Some(ref files) = self.files {
        d.emit_pair(b"files", files)?;
      }
      if let Some(ref profiles) = self.profiles {
        d.emit_pair(b"profiles", profiles)?;
      }
      Ok(())
    })
  }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "testx", derive(Dummy))]
pub struct File {
  pub length: i32,
  pub path: Vec<String>,
  #[serde(default)]
  pub md5sum: Option<String>,
}

impl FromBencode for File {
  fn decode_bencode_object(object: decoding::Object) -> Result<Self, decoding::Error>
  where
    Self: Sized,
  {
    let mut dict = object.try_into_dictionary()?;
    let mut length = None;
    let mut path = None;
    let mut md5sum = None;
    while let Some(pair) = dict.next_pair()? {
      match pair {
        (b"length", value) => {
          length = i32::decode_bencode_object(value)
            .context("length")
            .map(Some)?;
        },
        (b"path", value) => {
          path = Vec::<String>::decode_bencode_object(value)
            .context("path")
            .map(Some)?;
        },
        (b"md5sum", value) => {
          md5sum = String::decode_bencode_object(value)
            .context("md5sum")
            .map(Some)?;
        },
        (unknown_field, _) => {
          return Err(decoding::Error::unexpected_field(String::from_utf8_lossy(
            unknown_field,
          )));
        },
      }
    }
    Ok(Self {
      length: length.ok_or_else(|| decoding::Error::missing_field("length"))?,
      path: path.ok_or_else(|| decoding::Error::missing_field("path"))?,
      md5sum,
    })
  }
}

impl ToBencode for File {
  const MAX_DEPTH: usize = 10;
  fn encode(&self, encoder: encoding::SingleItemEncoder) -> Result<(), encoding::Error> {
    encoder.emit_unsorted_dict(|d| {
      d.emit_pair(b"length", self.length)?;
      d.emit_pair(b"path", &self.path)?;
      if let Some(ref md5sum) = self.md5sum {
        d.emit_pair(b"md5sum", md5sum)?;
      }
      Ok(())
    })
  }
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Node {
  pub node: String,
  pub port: i32,
}

impl FromBencode for Node {
  fn decode_bencode_object(object: decoding::Object) -> Result<Self, decoding::Error>
  where
    Self: Sized,
  {
    let mut dict = object.try_into_dictionary()?;
    let mut node = None;
    let mut port = None;
    while let Some(pair) = dict.next_pair()? {
      match pair {
        (b"node", value) => {
          node = String::decode_bencode_object(value)
            .context("node")
            .map(Some)?;
        },
        (b"port", value) => {
          port = i32::decode_bencode_object(value)
            .context("port")
            .map(Some)?;
        },
        (unknown_field, _) => {
          return Err(decoding::Error::unexpected_field(String::from_utf8_lossy(
            unknown_field,
          )));
        },
      }
    }
    Ok(Self {
      node: node.ok_or_else(|| decoding::Error::missing_field("node"))?,
      port: port.ok_or_else(|| decoding::Error::missing_field("port"))?,
    })
  }
}

impl ToBencode for Node {
  const MAX_DEPTH: usize = 10;
  fn encode(&self, encoder: encoding::SingleItemEncoder) -> Result<(), encoding::Error> {
    encoder.emit_unsorted_dict(|d| {
      d.emit_pair(b"node", &self.node)?;
      d.emit_pair(b"port", self.port)?;
      Ok(())
    })
  }
}

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

impl FromBencode for TorrentProfile {
  fn decode_bencode_object(object: decoding::Object) -> Result<Self, decoding::Error>
  where
    Self: Sized,
  {
    let mut dict = object.try_into_dictionary()?;
    let mut acodec = None;
    let mut height = None;
    let mut vcodec = None;
    let mut width = None;
    while let Some(pair) = dict.next_pair()? {
      match pair {
        (b"acodec", value) => {
          acodec = String::decode_bencode_object(value)
            .context("acodec")
            .map(Some)?;
        },
        (b"height", value) => {
          height = i32::decode_bencode_object(value)
            .context("height")
            .map(Some)?;
        },
        (b"vcodec", value) => {
          vcodec = String::decode_bencode_object(value)
            .context("vcodec")
            .map(Some)?;
        },
        (b"width", value) => {
          width = i32::decode_bencode_object(value)
            .context("width")
            .map(Some)?;
        },
        (unknown_field, _) => {
          return Err(decoding::Error::unexpected_field(String::from_utf8_lossy(
            unknown_field,
          )));
        },
      }
    }
    Ok(Self {
      acodec,
      height: height.ok_or_else(|| decoding::Error::missing_field("height"))?,
      vcodec,
      width: width.ok_or_else(|| decoding::Error::missing_field("width"))?,
    })
  }
}

impl ToBencode for TorrentProfile {
  const MAX_DEPTH: usize = 10;
  fn encode(&self, encoder: encoding::SingleItemEncoder) -> Result<(), encoding::Error> {
    encoder.emit_unsorted_dict(|d| {
      if let Some(ref acodec) = self.acodec {
        d.emit_pair(b"acodec", acodec)?;
      }
      d.emit_pair(b"height", self.height)?;
      if let Some(ref vcodec) = self.vcodec {
        d.emit_pair(b"vcodec", vcodec)?;
      }
      d.emit_pair(b"width", self.width)?;
      Ok(())
    })
  }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Validate, ToSchema)]
pub struct TorrentPatchDTO {
  #[serde(default)]
  pub nfo: Option<String>,
  #[serde(default)]
  pub genre: Option<Genre>,
}
