use core::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub const DEFAULT_TORRENT_FILTER_LIMIT: i64 = 50;

#[derive(Debug, Serialize, Deserialize)]
pub struct TorrentOrderBy {
    pub field: TorrentOrderByField,
    pub order: TorrentOrder,
}

impl fmt::Display for TorrentOrderBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("ORDER BY {} {}", self.field, self.order))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TorrentOrderByField {
    UploadedAt,
}

impl fmt::Display for TorrentOrderByField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UploadedAt => f.write_str("uploaded_at"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TorrentOrder {
    Asc,
    Desc,
}

impl fmt::Display for TorrentOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Asc => f.write_str("ASC"),
            Self::Desc => f.write_str("DESC"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TorrentFilter {
    /// If [`None`] [`DateTime::<Utc>::MIN_UTC`] is used.
    pub uploaded_at_min: Option<DateTime<Utc>>,
    /// If [`None`] [`DateTime::<Utc>::MAX_UTC`] is used.
    pub uploaded_at_max: Option<DateTime<Utc>>,
    /// If [`None`] no filter is applied.
    /// Otherwise filter by username.
    pub uploaded_by: Option<String>,
    /// If [`None`] no filter is applied.
    pub order_by: Option<TorrentOrderBy>,
    /// How many torrents to return.
    /// Defaults to [`DEFAULT_TORRENT_FILTER_LIMIT`] if [`None`]
    pub limit: Option<i64>,
}
