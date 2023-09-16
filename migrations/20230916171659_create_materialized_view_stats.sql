CREATE MATERIALIZED VIEW IF NOT EXISTS "PeerStats" AS
SELECT SUM(downloaded_bytes)::bigint AS downloaded_total,
       SUM(uploaded_bytes)::bigint   AS uploaded_total,
       SUM(left_bytes)::bigint       AS left_total,
       COUNT(*)::bigint              AS peers_total
FROM "Peer";

CREATE MATERIALIZED VIEW IF NOT EXISTS "TorrentStats" AS
SELECT SUM(length(raw))::bigint AS bytes_total,
       COUNT(*)::bigint         AS torrents_total
FROM "Torrent";

CREATE MATERIALIZED VIEW IF NOT EXISTS "UserStats" AS
SELECT role, COUNT(*)::bigint AS users_total
FROM "User"
GROUP BY role
ORDER BY users_total DESC;