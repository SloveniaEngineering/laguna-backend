use const_format::formatcp;

pub const APPLICATION_XBITTORRENT: &str = "application/x-bittorrent";
pub const APPLICATION_LAGUNA_JSON_VERSIONED: &str = formatcp!(
  "application/vnd.sloveniaengineering.laguna.{}+json",
  env!("CARGO_PKG_VERSION")
);
