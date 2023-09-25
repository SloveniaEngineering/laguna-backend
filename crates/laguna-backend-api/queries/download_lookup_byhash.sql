SELECT
  info_hash,
  user_id,
  ts,
  down_hash
FROM "Download"
WHERE down_hash = $1;