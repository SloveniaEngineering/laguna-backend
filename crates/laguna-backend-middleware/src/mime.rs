use const_format::formatcp;

/// MIME for `application/x-bittorrent` content type.
pub const APPLICATION_XBITTORRENT: &str = "application/x-bittorrent";

/// MIME for laguna-vendored `application/json` content type.
pub const APPLICATION_LAGUNA_JSON_VERSIONED: &str = formatcp!(
  "application/vnd.sloveniaengineering.laguna.{}+json",
  env!("CARGO_PKG_VERSION")
);
