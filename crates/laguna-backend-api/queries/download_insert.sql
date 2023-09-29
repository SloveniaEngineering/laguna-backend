INSERT INTO "Download"  (
  info_hash,
  user_id,
  ts,
  down_hash
)
VALUES (
  $1,
  $2,
  $3,
  $4
)
RETURNING info_hash,
          user_id,
          ts,
          down_hash;
