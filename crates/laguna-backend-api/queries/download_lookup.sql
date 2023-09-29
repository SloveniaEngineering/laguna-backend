SELECT
  info_hash,
  user_id,
  ts,
  down_hash
FROM "Download"
WHERE info_hash = $1
  AND user_id = $2